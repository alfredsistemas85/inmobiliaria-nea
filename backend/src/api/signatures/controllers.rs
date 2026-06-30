use super::dto::{CreateSignatureRequestDto, InitSignaturesDto, SubmitSignatureDto};
use super::services::SignatureService;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn request_signature(
    State(pool): State<Arc<PgPool>>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::core::security::jwt::Claims>,
    Path(contract_id): Path<Uuid>,
    Json(payload): Json<InitSignaturesDto>,
) -> impl IntoResponse {
    let tenant_id = match claims.tenant_id {
        Some(t) => t,
        None => return (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "success": false, "error": "Missing tenant_id" }))).into_response(),
    };
    let user_id = claims.sub;

    match SignatureService::create_requests(&pool, tenant_id, contract_id, payload.requests, user_id).await {
        Ok(res) => Json(serde_json::json!({ "success": true, "data": res })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e })),
        )
            .into_response(),
    }
}

pub async fn submit_signature(
    State(pool): State<Arc<PgPool>>,
    Path(token): Path<String>,
    Json(payload): Json<SubmitSignatureDto>,
) -> impl IntoResponse {
    match SignatureService::submit_signature(&pool, &token, payload).await {
        Ok(res) => Json(serde_json::json!({ "success": true, "data": res })).into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_public_info(
    State(pool): State<Arc<PgPool>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    let mut hasher = sha2::Sha256::new();
    use sha2::Digest;
    hasher.update(token.as_bytes());
    let token_hash = hex::encode(hasher.finalize());

    if let Ok(Some(req)) = super::repository::SignatureRepository::get_request_by_token_hash(&pool, &token_hash).await {
        let mut snapshot = super::repository::SignatureRepository::get_snapshot_by_contract(&pool, req.contract_id)
            .await
            .unwrap_or(None);

        if snapshot.is_none() {
            let repo = crate::infrastructure::database::contracts::ContractRepository::new(pool.clone());
            if let Ok(live_data) = repo.get_contract(req.tenant_id, req.contract_id).await {
                snapshot = Some(live_data);
            }
        }

        return Json(serde_json::json!({
            "success": true,
            "data": {
                "contract_id": req.contract_id,
                "participant_id": req.participant_id,
                "status": req.status,
                "expires_at": req.expires_at,
                "contract_snapshot": snapshot
            }
        })).into_response();
    }
    
    (
        axum::http::StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "success": false, "error": "Token not found" })),
    ).into_response()
}

pub async fn verify_signature(
    State(pool): State<Arc<PgPool>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    if let Ok(Some(req)) = super::repository::SignatureRepository::get_request_by_verification_code(&pool, &code).await {
        return Json(serde_json::json!({
            "success": true,
            "data": {
                "valid": true,
                "contract_id": req.contract_id,
                "status": req.status,
                "signed_at": req.signed_at
            }
        })).into_response();
    }
    
    (
        axum::http::StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "success": false, "error": "Verification code not found" })),
    ).into_response()
}

// ... other administrative endpoints (cancel, resend, regenerate) follow the same pattern
