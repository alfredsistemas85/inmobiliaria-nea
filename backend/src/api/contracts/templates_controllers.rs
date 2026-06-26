use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;

use super::dto::CreateContractTemplateDto;
use super::models::{ContractTemplate, TemplateClause};
use crate::core::security::jwt::Claims;
use uuid::Uuid;

pub async fn list_templates(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims
        .tenant_id
        .ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    let templates: Vec<ContractTemplate> = sqlx::query_as(
        "SELECT * FROM contract_templates WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC"
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching templates: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error obteniendo plantillas".to_string())
    })?;

    Ok(Json(serde_json::json!(templates)))
}

pub async fn create_template(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContractTemplateDto>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims
        .tenant_id
        .ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let template = sqlx::query_as::<_, ContractTemplate>(
        r#"
        INSERT INTO contract_templates (tenant_id, name, description, c_type, c_destination, created_by, updated_by)
        VALUES ($1, $2, $3, $4, $5, $6, $6)
        RETURNING *
        "#
    )
    .bind(tenant_id)
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.c_type.clone())
    .bind(payload.c_destination.clone())
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error creando template: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error creando plantilla".to_string())
    })?;

    for clause in payload.clauses {
        sqlx::query(
            r#"
            INSERT INTO template_clauses (tenant_id, template_id, display_order, title, body, is_mandatory, is_editable, is_system, created_by, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
            "#
        )
        .bind(tenant_id)
        .bind(template.id)
        .bind(clause.display_order)
        .bind(clause.title)
        .bind(clause.body)
        .bind(clause.is_mandatory.unwrap_or(false))
        .bind(clause.is_editable.unwrap_or(true))
        .bind(clause.is_system.unwrap_or(false))
        .bind(claims.sub)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Error insertando clausula de plantilla: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error insertando clausula".to_string())
        })?;
    }

    tx.commit().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error en commit".to_string(),
        )
    })?;

    Ok(Json(serde_json::json!(template)))
}

pub async fn get_template(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims
        .tenant_id
        .ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    let row: (serde_json::Value,) = sqlx::query_as(
        r#"
        SELECT json_build_object(
            'template', row_to_json(t.*),
            'clauses', (
                SELECT COALESCE(json_agg(row_to_json(c.*) ORDER BY c.display_order ASC), '[]'::json)
                FROM template_clauses c
                WHERE c.template_id = t.id AND c.deleted_at IS NULL
            )
        )
        FROM contract_templates t
        WHERE t.id = $1 AND t.tenant_id = $2 AND t.deleted_at IS NULL
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(row.0))
}

pub async fn delete_template(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims
        .tenant_id
        .ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    sqlx::query(
        "UPDATE contract_templates SET deleted_at = NOW(), deleted_by = $1 WHERE id = $2 AND tenant_id = $3"
    )
    .bind(claims.sub)
    .bind(id)
    .bind(tenant_id)
    .execute(&*pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({"status": "deleted"})))
}
