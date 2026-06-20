use crate::{api::roles::dtos::RoleResponseDto, models::role::Role};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn list_roles(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<RoleResponseDto>>, StatusCode> {
    let roles = sqlx::query_as::<_, Role>("SELECT id, name, description FROM roles")
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(roles.into_iter().map(RoleResponseDto::from).collect()))
}
