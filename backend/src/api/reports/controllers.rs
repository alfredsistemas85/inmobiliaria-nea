use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Extension,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{core::security::jwt::Claims, infrastructure::database::audit::AuditRepository};

#[derive(Deserialize)]
pub struct ReportFilters {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
}

pub async fn generate_leads_report(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<ReportFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    // Fetch leads with optional filters
    let leads = sqlx::query!(
        r#"
        SELECT id, client_id, property_id, status, source, created_at, assigned_to
        FROM leads
        WHERE tenant_id = $1 AND deleted_at IS NULL
          AND ($2::timestamptz IS NULL OR created_at >= $2)
          AND ($3::timestamptz IS NULL OR created_at <= $3)
          AND ($4::uuid IS NULL OR assigned_to = $4)
        ORDER BY created_at DESC
        "#,
        tenant_id,
        filters.date_from,
        filters.date_to,
        filters.assigned_to
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Status", "Source", "Created At", "Assigned To"])
        .unwrap();
    for lead in leads {
        wtr.write_record(&[
            lead.id.to_string(),
            lead.status.unwrap_or_default(),
            lead.source.unwrap_or_default(),
            lead.created_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            lead.assigned_to.map(|u| u.to_string()).unwrap_or_default(),
        ])
        .unwrap();
    }

    let csv_data = wtr.into_inner().unwrap();

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "REPORT_EXPORTED",
            "reports",
            None,
            Some(serde_json::json!({"type": "leads"})),
        )
        .await;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"leads_report.csv\"",
            ),
        ],
        csv_data,
    ))
}

pub async fn generate_appointments_report(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<ReportFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let appointments = sqlx::query!(
        r#"
        SELECT id, client_id, property_id, status, scheduled_at, created_at, user_id as assigned_to
        FROM appointments
        WHERE tenant_id = $1 AND deleted_at IS NULL
          AND ($2::timestamptz IS NULL OR scheduled_at >= $2)
          AND ($3::timestamptz IS NULL OR scheduled_at <= $3)
          AND ($4::uuid IS NULL OR user_id = $4)
        ORDER BY scheduled_at DESC
        "#,
        tenant_id,
        filters.date_from,
        filters.date_to,
        filters.assigned_to
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Status", "Scheduled At", "Created At", "Assigned To"])
        .unwrap();
    for app in appointments {
        wtr.write_record(&[
            app.id.to_string(),
            app.status.unwrap_or_default(),
            app.scheduled_at.to_rfc3339(),
            app.created_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            app.assigned_to.map(|u| u.to_string()).unwrap_or_default(),
        ])
        .unwrap();
    }

    let csv_data = wtr.into_inner().unwrap();

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "REPORT_EXPORTED",
            "reports",
            None,
            Some(serde_json::json!({"type": "appointments"})),
        )
        .await;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"appointments_report.csv\"",
            ),
        ],
        csv_data,
    ))
}

pub async fn generate_whatsapp_report(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<ReportFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let conversations = sqlx::query!(
        r#"
        SELECT id, client_id, status, created_at, last_message_at, assigned_user_id
        FROM conversations
        WHERE tenant_id = $1 AND deleted_at IS NULL
          AND ($2::timestamptz IS NULL OR last_message_at >= $2)
          AND ($3::timestamptz IS NULL OR last_message_at <= $3)
          AND ($4::uuid IS NULL OR assigned_user_id = $4)
        ORDER BY last_message_at DESC NULLS LAST
        "#,
        tenant_id,
        filters.date_from,
        filters.date_to,
        filters.assigned_to
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        "ID",
        "Status",
        "Created At",
        "Last Message At",
        "Assigned To",
    ])
    .unwrap();
    for conv in conversations {
        wtr.write_record(&[
            conv.id.to_string(),
            conv.status.unwrap_or_default(),
            conv.created_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            conv.last_message_at
                .map(|d| d.to_rfc3339())
                .unwrap_or_default(),
            conv.assigned_user_id
                .map(|u| u.to_string())
                .unwrap_or_default(),
        ])
        .unwrap();
    }

    let csv_data = wtr.into_inner().unwrap();

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "REPORT_EXPORTED",
            "reports",
            None,
            Some(serde_json::json!({"type": "whatsapp"})),
        )
        .await;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"whatsapp_report.csv\"",
            ),
        ],
        csv_data,
    ))
}

pub async fn generate_clients_report(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<ReportFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let clients = sqlx::query!(
        r#"
        SELECT id, first_name, last_name, email, phone, created_at
        FROM clients
        WHERE tenant_id = $1 AND deleted_at IS NULL
          AND ($2::timestamptz IS NULL OR created_at >= $2)
          AND ($3::timestamptz IS NULL OR created_at <= $3)
        ORDER BY created_at DESC
        "#,
        tenant_id,
        filters.date_from,
        filters.date_to
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Name", "Email", "Phone", "Created At"])
        .unwrap();
    for client in clients {
        let name = format!(
            "{} {}",
            client.first_name.unwrap_or_default(),
            client.last_name.unwrap_or_default()
        );
        wtr.write_record(&[
            client.id.to_string(),
            name.trim().to_string(),
            client.email.unwrap_or_default(),
            client.phone, // phone is String
            client
                .created_at
                .map(|d: chrono::DateTime<chrono::Utc>| d.to_rfc3339())
                .unwrap_or_default(),
        ])
        .unwrap();
    }

    let csv_data = wtr.into_inner().unwrap();

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "REPORT_EXPORTED",
            "reports",
            None,
            Some(serde_json::json!({"type": "clients"})),
        )
        .await;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"clients_report.csv\"",
            ),
        ],
        csv_data,
    ))
}

pub async fn generate_properties_report(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<ReportFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let properties = sqlx::query!(
        r#"
        SELECT id, title, property_type, price, status, created_at
        FROM properties
        WHERE tenant_id = $1 AND deleted_at IS NULL
          AND ($2::timestamptz IS NULL OR created_at >= $2)
          AND ($3::timestamptz IS NULL OR created_at <= $3)
        ORDER BY created_at DESC
        "#,
        tenant_id,
        filters.date_from,
        filters.date_to
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Title", "Type", "Price", "Status", "Created At"])
        .unwrap();
    for prop in properties {
        wtr.write_record(&[
            prop.id.to_string(),
            prop.title,
            prop.property_type,
            prop.price.to_string(), // price is not null
            prop.status.unwrap_or_default(),
            prop.created_at
                .map(|d: chrono::DateTime<chrono::Utc>| d.to_rfc3339())
                .unwrap_or_default(),
        ])
        .unwrap();
    }

    let csv_data = wtr.into_inner().unwrap();

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "REPORT_EXPORTED",
            "reports",
            None,
            Some(serde_json::json!({"type": "properties"})),
        )
        .await;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"properties_report.csv\"",
            ),
        ],
        csv_data,
    ))
}
