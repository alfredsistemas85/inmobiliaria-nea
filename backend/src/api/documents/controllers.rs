use super::storage::SupabaseStorage;
use crate::core::security::jwt::Claims;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub mime_type: String,
    pub storage_path: Option<String>,
    pub version: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateUploadUrlDto {
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
}

#[derive(Serialize)]
pub struct UploadUrlResponse {
    pub document_id: Uuid,
    pub upload_url: String,
    pub storage_path: String,
}

pub async fn generate_upload_url(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUploadUrlDto>,
) -> Result<Json<UploadUrlResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;
    let user_id = claims.sub;

    let storage_path = format!(
        "{}/{}/{}/{}",
        tenant_id, payload.entity_type, payload.entity_id, payload.file_name
    );

    let storage = SupabaseStorage::new();
    let upload_url = storage
        .create_upload_url(&storage_path)
        .await
        .map_err(|e| {
            tracing::error!("Storage Error: {}", e);
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    // Registrar en BD
    let doc = sqlx::query_as::<_, Document>(
        r#"
        INSERT INTO documents (tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path, version, created_at
        "#
    )
    .bind(tenant_id)
    .bind(user_id)
    .bind(payload.entity_type)
    .bind(payload.entity_id)
    .bind(payload.file_name)
    .bind(payload.file_size)
    .bind(payload.mime_type)
    .bind(storage_path.clone())
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("DB Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Registrar log
    let _ = sqlx::query(
        "INSERT INTO document_access_logs (tenant_id, user_id, document_id, action) VALUES ($1, $2, $3, 'UPLOAD')"
    )
    .bind(tenant_id)
    .bind(user_id)
    .bind(doc.id)
    .execute(&*pool)
    .await;

    Ok(Json(UploadUrlResponse {
        document_id: doc.id,
        upload_url,
        storage_path,
    }))
}

pub async fn list_entity_documents(
    State(pool): State<Arc<PgPool>>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Document>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let docs = sqlx::query_as::<_, Document>(
        "SELECT id, tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path, version, created_at FROM documents WHERE tenant_id = $1 AND entity_type = $2 AND entity_id = $3 AND deleted_at IS NULL ORDER BY created_at DESC"
    )
    .bind(tenant_id)
    .bind(entity_type)
    .bind(entity_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error en list_entity_documents: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(docs))
}

#[derive(Serialize)]
pub struct DownloadUrlResponse {
    pub url: String,
}

pub async fn get_document(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<DownloadUrlResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let path_row = sqlx::query("SELECT storage_path FROM documents WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL")
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let storage_path: String = path_row.try_get("storage_path").unwrap_or_default();

    let storage = SupabaseStorage::new();
    let url = storage
        .create_download_url(&storage_path, 3600)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    // Registrar log
    let _ = sqlx::query(
        "INSERT INTO document_access_logs (tenant_id, user_id, document_id, action) VALUES ($1, $2, $3, 'VIEW')"
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(id)
    .execute(&*pool)
    .await;

    Ok(Json(DownloadUrlResponse { url }))
}

pub async fn delete_document(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let result = sqlx::query(
        "UPDATE documents SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
    )
    .bind(id)
    .bind(tenant_id)
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let _ = sqlx::query(
        "INSERT INTO document_access_logs (tenant_id, user_id, document_id, action) VALUES ($1, $2, $3, 'DELETE')"
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(id)
    .execute(&*pool)
    .await;

    Ok(StatusCode::OK)
}

pub async fn view_document(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<axum::response::Response, StatusCode> {
    let path_row = sqlx::query(
        "SELECT storage_path, mime_type FROM documents WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let storage_path: String = path_row.try_get("storage_path").unwrap_or_default();
    let mime_type: String = path_row
        .try_get("mime_type")
        .unwrap_or_else(|_| "application/octet-stream".to_string());

    let storage = SupabaseStorage::new();
    let url = storage
        .create_download_url(&storage_path, 3600)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if !res.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    let bytes = res
        .bytes()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    axum::response::Response::builder()
        .header("Content-Type", mime_type)
        .header("Cache-Control", "public, max-age=31536000")
        .body(axum::body::Body::from(bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
