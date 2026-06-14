use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Extension,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::json;

use crate::{
    api::appointments::dtos::{CreateAppointmentDto, UpdateAppointmentDto},
    core::security::jwt::Claims,
    infrastructure::database::{appointments::AppointmentRepository, audit::AuditRepository, notifications::NotificationRepository},
    models::{appointment::Appointment, common::{PaginatedResponse, PaginationParams}},
};

pub async fn list_appointments(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Appointment>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = AppointmentRepository::new(pool);
    
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    let q = params.q.as_deref();

    let appointments = repo.list(tenant_id, limit, offset, q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(appointments))
}

pub async fn get_appointment(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Appointment>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = AppointmentRepository::new(pool);
    
    let appointment = repo.get_by_id(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(appointment))
}

pub async fn create_appointment(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAppointmentDto>,
) -> Result<Json<Appointment>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = AppointmentRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool.clone());

    let appointment = repo.create(
        tenant_id,
        payload.client_id,
        payload.property_id,
        payload.user_id,
        payload.scheduled_at,
        payload.status.as_deref(),
        payload.notes.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = audit_repo.log(
        Some(tenant_id),
        Some(claims.sub),
        "CREATE_APPOINTMENT",
        "appointment",
        Some(appointment.id),
        Some(json!({ "scheduled_at": appointment.scheduled_at })),
    ).await;

    if appointment.user_id.is_some() {
        let notif_repo = NotificationRepository::new(pool.clone());
        let _ = notif_repo.create(
            tenant_id,
            appointment.user_id,
            "APPOINTMENT_ASSIGNED",
            "Cita Asignada",
            &format!("Se te ha asignado una nueva cita para el {}", appointment.scheduled_at),
        ).await;
    }

    Ok(Json(appointment))
}

pub async fn update_appointment(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateAppointmentDto>,
) -> Result<Json<Appointment>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = AppointmentRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let appointment = repo.update(
        id,
        tenant_id,
        payload.client_id,
        payload.property_id,
        payload.user_id,
        payload.scheduled_at,
        payload.status.as_deref(),
        payload.notes.as_deref(),
    )
    .await
    .map_err(|e| {
        match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    let _ = audit_repo.log(
        Some(tenant_id),
        Some(claims.sub),
        "UPDATE_APPOINTMENT",
        "appointment",
        Some(appointment.id),
        None,
    ).await;

    Ok(Json(appointment))
}

pub async fn delete_appointment(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = AppointmentRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let rows_affected = repo.soft_delete(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        let _ = audit_repo.log(
            Some(tenant_id),
            Some(claims.sub),
            "DELETE_APPOINTMENT",
            "appointment",
            Some(id),
            None,
        ).await;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
