use super::dto::{CreateSignatureRequestDto, SignatureRequestResponseDto, SubmitSignatureDto, SubmitSignatureResponseDto, VerificationResponseDto};
use super::models::{ContractSignatureRequest, SignatureRequestStatus, SignatureType};
use super::repository::SignatureRepository;
use crate::api::documents::storage::SupabaseStorage;
use crate::core::contracts::signed_pdf_generator::SignedPdfGenerator;
use crate::infrastructure::evolution::client::EvolutionClient;
use chrono::{DateTime, Duration, Utc};
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

pub struct SignatureService;

impl SignatureService {
    pub fn generate_secure_token() -> (String, String) {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let token_plain = hex::encode(key);

        let mut hasher = Sha256::new();
        hasher.update(token_plain.as_bytes());
        let token_hash = hex::encode(hasher.finalize());

        (token_plain, token_hash)
    }

    pub fn generate_verification_code() -> String {
        let mut key = [0u8; 4];
        OsRng.fill_bytes(&mut key);
        hex::encode(key).to_uppercase()
    }

    pub fn hash_content(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    pub async fn upload_base64_to_storage(
        base64_data: &str,
        path: &str,
        mime_type: &str,
    ) -> Result<String, String> {
        let storage = SupabaseStorage::new();
        let bytes = base64::decode(base64_data).map_err(|e| e.to_string())?;
        
        let upload_url = storage.create_upload_url(path).await.map_err(|e| e.to_string())?;
        let client = reqwest::Client::new();
        let resp = client.put(&upload_url)
            .header("Content-Type", mime_type)
            .body(bytes)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Error uploading to Supabase: {}", resp.status()));
        }

        Ok(path.to_string())
    }

    pub async fn upload_pdf_to_storage(
        pdf_bytes: Vec<u8>,
        path: &str,
    ) -> Result<String, String> {
        let storage = SupabaseStorage::new();
        let upload_url = storage.create_upload_url(path).await.map_err(|e| e.to_string())?;
        let client = reqwest::Client::new();
        let resp = client.put(&upload_url)
            .header("Content-Type", "application/pdf")
            .body(pdf_bytes)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Error uploading PDF to Supabase: {}", resp.status()));
        }
        Ok(path.to_string())
    }

    pub async fn create_requests(
        pool: &PgPool,
        tenant_id: Uuid,
        contract_id: Uuid,
        requests: Vec<CreateSignatureRequestDto>,
        user_id: Uuid,
    ) -> Result<Vec<SignatureRequestResponseDto>, String> {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Get snapshot (if it doesn't exist, create it)
        let existing_snapshot = SignatureRepository::get_snapshot_by_contract(pool, contract_id).await.unwrap_or(None);
        if existing_snapshot.is_none() {
            // Generate dummy snapshot for now (In real app, fetch full contract data)
            let snap = serde_json::json!({
                "contract": { "id": contract_id, "start_date": "2026-07-01", "original_rent_amount": 10000.0 },
                "clauses": [
                    { "title": "Objeto", "body": "Se alquila el inmueble..." }
                ]
            });
            SignatureRepository::insert_snapshot(&mut tx, tenant_id, contract_id, snap.clone()).await.map_err(|e| format!("tenant_id: {}, contract_id: {}, error: {}", tenant_id, contract_id, e))?;
            
            // Generate original PDF
            let generator = SignedPdfGenerator::new("assets/fonts").unwrap();
            let original_pdf = generator.generate_signed_contract(snap, vec![]).await?;
            let sha256 = Self::hash_content(&original_pdf);
            let path = format!("{}/{}/original.pdf", tenant_id, contract_id);
            Self::upload_pdf_to_storage(original_pdf, &path).await?;
            SignatureRepository::insert_contract_document(&mut tx, tenant_id, contract_id, "ORIGINAL", &path, "application/pdf", &sha256, user_id).await.map_err(|e| e.to_string())?;
        }

        // Update contract status to READY_FOR_SIGNATURE if currently DRAFT
        SignatureRepository::update_contract_status(&mut tx, contract_id, "READY_FOR_SIGNATURE").await.map_err(|e| e.to_string())?;

        let mut responses = Vec::new();

        let expiration_days = 7; // In a real app, read from system_settings here
        let expires_at = Utc::now() + Duration::days(expiration_days);

        for req in requests {
            let (token_plain, token_hash) = Self::generate_secure_token();
            let verification_code = Self::generate_verification_code();

            let request_id = SignatureRepository::insert_signature_request(
                &mut tx,
                tenant_id,
                contract_id,
                req.participant_id,
                &token_hash,
                &verification_code,
                req.signature_order,
                req.required_signature,
                req.signature_type.unwrap_or(SignatureType::Handdrawn),
                Some(expires_at),
                user_id
            ).await.map_err(|e| e.to_string())?;

            SignatureRepository::insert_event(
                &mut tx,
                tenant_id,
                request_id,
                "REQUEST_CREATED",
                None,
                None,
                Some(user_id)
            ).await.map_err(|e| e.to_string())?;

            responses.push(SignatureRequestResponseDto {
                id: request_id,
                contract_id,
                participant_id: req.participant_id,
                status: SignatureRequestStatus::Pending,
                signature_order: req.signature_order,
                required_signature: req.required_signature,
                expires_at: Some(expires_at),
                link: Some(format!("https://inmonea.agentech.ar/s/{}", token_plain)),
            });

            // Send WhatsApp async
            // tokio::spawn(async move { ... EvolutionClient::new().send_message(...) });
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(responses)
    }

    pub async fn submit_signature(
        pool: &PgPool,
        token_plain: &str,
        dto: SubmitSignatureDto,
    ) -> Result<SubmitSignatureResponseDto, String> {
        let mut hasher = Sha256::new();
        hasher.update(token_plain.as_bytes());
        let token_hash = hex::encode(hasher.finalize());

        let req = SignatureRepository::get_request_by_token_hash(pool, &token_hash)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Invalid or expired token")?;

        if req.status == SignatureRequestStatus::Signed {
            return Err("Ya fue firmado".into());
        }

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Upload image to Storage
        let path = format!("{}/{}/signatures/{}.png", req.tenant_id, req.contract_id, req.id);
        let uploaded_path = Self::upload_base64_to_storage(&dto.signature_base64, &path, "image/png").await?;
        
        let signature_sha256 = Self::hash_content(&base64::decode(&dto.signature_base64).unwrap());

        // 2. Insert signature
        let signature_id = SignatureRepository::insert_signature(
            &mut tx,
            req.tenant_id,
            req.contract_id,
            req.participant_id,
            req.id,
            Some(uploaded_path),
            Some(signature_sha256),
            dto.browser,
            dto.operating_system,
            dto.ip,
            dto.user_agent,
            dto.latitude,
            dto.longitude
        ).await.map_err(|e| e.to_string())?;

        // 3. Update request status
        SignatureRepository::update_request_status(&mut tx, req.id, SignatureRequestStatus::Signed).await.map_err(|e| e.to_string())?;

        SignatureRepository::insert_event(
            &mut tx,
            req.tenant_id,
            req.id,
            "SIGNED",
            None,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        // 4. Check if all required are signed
        let all_requests = SignatureRepository::get_requests_by_contract(pool, req.contract_id).await.unwrap_or_default();
        let all_mandatory_signed = all_requests.iter()
            .filter(|r| r.required_signature)
            .all(|r| r.id == req.id || r.status == SignatureRequestStatus::Signed);

        if all_mandatory_signed {
            // Create final PDF
            let snap = SignatureRepository::get_snapshot_by_contract(pool, req.contract_id).await.unwrap().unwrap_or(serde_json::json!({}));
            
            // Gather signatures metadata for PDF
            let mut sig_values = vec![];
            sig_values.push(serde_json::json!({
                "name": "Firmante",
                "signed_at": Utc::now().to_string(),
                "ip": "127.0.0.1",
                "browser": "Chrome",
                "signature_sha256": "abcdef",
                "verification_code": req.verification_code
            }));

            let generator = SignedPdfGenerator::new("assets/fonts").unwrap();
            let final_pdf = generator.generate_signed_contract(snap, sig_values).await?;
            let sha256 = Self::hash_content(&final_pdf);
            
            let final_path = format!("{}/{}/final_signed.pdf", req.tenant_id, req.contract_id);
            Self::upload_pdf_to_storage(final_pdf, &final_path).await?;
            
            // Insert document
            SignatureRepository::insert_contract_document(&mut tx, req.tenant_id, req.contract_id, "FINAL_SIGNED", &final_path, "application/pdf", &sha256, Uuid::default()).await.map_err(|e| e.to_string())?;

            // Update contract status
            SignatureRepository::update_contract_status(&mut tx, req.contract_id, "ACTIVE").await.map_err(|e| e.to_string())?;
        } else {
            SignatureRepository::update_contract_status(&mut tx, req.contract_id, "SIGNING").await.map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(SubmitSignatureResponseDto {
            success: true,
            message: "Firma registrada exitosamente".to_string()
        })
    }
}
