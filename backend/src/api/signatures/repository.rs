use super::models::{
    ContractSignature, ContractSignatureEvent, ContractSignatureRequest, ContractSignatureSession,
    ContractSnapshot, SignatureRequestStatus,
};
use crate::api::signatures::models::SignatureType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction, Row};
use uuid::Uuid;

pub struct SignatureRepository;

impl SignatureRepository {
    pub async fn insert_snapshot(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_id: Uuid,
        snapshot_json: serde_json::Value,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_snapshots (tenant_id, contract_id, snapshot_json)
            VALUES ($1, $2, $3)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(contract_id)
        .bind(snapshot_json)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn insert_contract_document(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_id: Uuid,
        document_type: &str,
        storage_path: &str,
        mime_type: &str,
        sha256: &str,
        created_by: Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_documents (tenant_id, contract_id, document_type, storage_path, file_path, mime_type, sha256, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(contract_id)
        .bind(document_type)
        .bind(storage_path)
        .bind(storage_path)
        .bind(mime_type)
        .bind(sha256)
        .bind(created_by)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn insert_signature_request(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_id: Uuid,
        participant_id: Uuid,
        token_hash: &str,
        verification_code: &str,
        signature_order: i32,
        required_signature: bool,
        signature_type: SignatureType,
        expires_at: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_signature_requests 
                (tenant_id, contract_id, participant_id, token_hash, verification_code, signature_order, required_signature, signature_type, expires_at, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(contract_id)
        .bind(participant_id)
        .bind(token_hash)
        .bind(verification_code)
        .bind(signature_order)
        .bind(required_signature)
        .bind(signature_type as SignatureType)
        .bind(expires_at)
        .bind(created_by)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn insert_event(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        request_id: Uuid,
        event_type: &str,
        description: Option<String>,
        metadata: Option<serde_json::Value>,
        created_by: Option<Uuid>,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_signature_events (tenant_id, request_id, event_type, description, metadata, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(request_id)
        .bind(event_type)
        .bind(description)
        .bind(metadata)
        .bind(created_by)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn get_request_by_token_hash(
        pool: &PgPool,
        token_hash: &str,
    ) -> Result<Option<ContractSignatureRequest>, sqlx::Error> {
        sqlx::query_as::<_, ContractSignatureRequest>(
            r#"
            SELECT 
                id, tenant_id, contract_id, participant_id, token_hash, verification_code,
                signature_order, required_signature, signature_type,
                status, expires_at, opened_at, viewed_at, signed_at,
                created_at, updated_at, deleted_at, created_by, updated_by, deleted_by
            FROM contract_signature_requests
            WHERE token_hash = $1 AND deleted_at IS NULL
            "#
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_request_by_verification_code(
        pool: &PgPool,
        verification_code: &str,
    ) -> Result<Option<ContractSignatureRequest>, sqlx::Error> {
        sqlx::query_as::<_, ContractSignatureRequest>(
            r#"
            SELECT 
                id, tenant_id, contract_id, participant_id, token_hash, verification_code,
                signature_order, required_signature, signature_type,
                status, expires_at, opened_at, viewed_at, signed_at,
                created_at, updated_at, deleted_at, created_by, updated_by, deleted_by
            FROM contract_signature_requests
            WHERE verification_code = $1 AND deleted_at IS NULL
            "#
        )
        .bind(verification_code)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_request_status(
        tx: &mut Transaction<'_, Postgres>,
        request_id: Uuid,
        status: SignatureRequestStatus,
    ) -> Result<(), sqlx::Error> {
        let mut q = "UPDATE contract_signature_requests SET status = $1, updated_at = CURRENT_TIMESTAMP".to_string();
        if status == SignatureRequestStatus::Opened {
            q.push_str(", opened_at = COALESCE(opened_at, CURRENT_TIMESTAMP)");
        } else if status == SignatureRequestStatus::Viewed {
            q.push_str(", viewed_at = COALESCE(viewed_at, CURRENT_TIMESTAMP)");
        } else if status == SignatureRequestStatus::Signed {
            q.push_str(", signed_at = CURRENT_TIMESTAMP");
        }
        
        q.push_str(" WHERE id = $2");

        sqlx::query(&q)
            .bind(status as SignatureRequestStatus)
            .bind(request_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn insert_signature(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_id: Uuid,
        participant_id: Uuid,
        request_id: Uuid,
        signature_image_path: Option<String>,
        signature_sha256: Option<String>,
        browser: Option<String>,
        os: Option<String>,
        ip: Option<String>,
        user_agent: Option<String>,
        latitude: Option<Decimal>,
        longitude: Option<Decimal>,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_signatures 
                (tenant_id, contract_id, participant_id, request_id, signature_image_path, signature_sha256, browser, operating_system, ip, user_agent, latitude, longitude)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(contract_id)
        .bind(participant_id)
        .bind(request_id)
        .bind(signature_image_path)
        .bind(signature_sha256)
        .bind(browser)
        .bind(os)
        .bind(ip)
        .bind(user_agent)
        .bind(latitude)
        .bind(longitude)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn insert_session(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        request_id: Uuid,
        browser: Option<String>,
        os: Option<String>,
        ip: Option<String>,
        user_agent: Option<String>,
        fingerprint: Option<String>,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO contract_signature_sessions 
                (tenant_id, request_id, browser, os, ip, user_agent, fingerprint)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(request_id)
        .bind(browser)
        .bind(os)
        .bind(ip)
        .bind(user_agent)
        .bind(fingerprint)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn get_requests_by_contract(
        pool: &PgPool,
        contract_id: Uuid,
    ) -> Result<Vec<ContractSignatureRequest>, sqlx::Error> {
        sqlx::query_as::<_, ContractSignatureRequest>(
            r#"
            SELECT 
                id, tenant_id, contract_id, participant_id, token_hash, verification_code,
                signature_order, required_signature, signature_type,
                status, expires_at, opened_at, viewed_at, signed_at,
                created_at, updated_at, deleted_at, created_by, updated_by, deleted_by
            FROM contract_signature_requests
            WHERE contract_id = $1 AND deleted_at IS NULL
            ORDER BY signature_order ASC
            "#
        )
        .bind(contract_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_contract_status(
        tx: &mut Transaction<'_, Postgres>,
        contract_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE contracts SET status = $1::contract_status, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(status)
            .bind(contract_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn get_snapshot_by_contract(
        pool: &PgPool,
        contract_id: Uuid,
    ) -> Result<Option<serde_json::Value>, sqlx::Error> {
        let snapshot_json = sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT snapshot_json FROM contract_snapshots WHERE contract_id = $1 ORDER BY created_at DESC LIMIT 1"
        )
        .bind(contract_id)
        .fetch_optional(pool)
        .await?;

        Ok(snapshot_json)
    }

    pub async fn update_request_token(
        tx: &mut Transaction<'_, Postgres>,
        request_id: Uuid,
        new_token_hash: &str,
        new_expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE contract_signature_requests 
            SET token_hash = $1, expires_at = $2, updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            "#
        )
        .bind(new_token_hash)
        .bind(new_expires_at)
        .bind(request_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
