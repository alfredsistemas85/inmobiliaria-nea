use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

use crate::core::security::jwt::Claims;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_clients: i64,
    pub total_properties: i64,
    pub new_leads: i64,
    pub upcoming_appointments: i64,
    pub active_whatsapp_conversations: i64,
    pub leads_this_month: i64,
    pub conversions_this_month: i64, // Leads moved to CLOSED or WON

    // New fields for charts
    pub leads_by_status: Vec<LeadStatusCount>,
    pub conversations_by_agent: Vec<AgentConversationCount>,
    pub conversions_by_month: Vec<MonthlyConversion>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct LeadStatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AgentConversationCount {
    pub agent_name: String,
    pub count: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MonthlyConversion {
    pub month: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct DashboardActivity {
    pub id: String,
    pub title: String,
    pub time: String,
    pub r#type: String, // lead, client, property, appointment
}

pub async fn get_stats(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<DashboardStats>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let p1 = pool.clone();
    let f_total_clients = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM clients WHERE tenant_id = $1 AND deleted_at IS NULL",
        )
        .bind(tenant_id)
        .fetch_one(&*p1)
        .await
    });

    let p2 = pool.clone();
    let f_total_properties = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM properties WHERE tenant_id = $1 AND deleted_at IS NULL",
        )
        .bind(tenant_id)
        .fetch_one(&*p2)
        .await
    });

    let p3 = pool.clone();
    let f_new_leads = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM leads WHERE tenant_id = $1 AND status = 'NUEVO' AND deleted_at IS NULL")
            .bind(tenant_id).fetch_one(&*p3).await
    });

    let p4 = pool.clone();
    let f_upcoming_appointments = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM appointments WHERE tenant_id = $1 AND scheduled_at >= CURRENT_TIMESTAMP AND deleted_at IS NULL")
            .bind(tenant_id).fetch_one(&*p4).await
    });

    let p5 = pool.clone();
    let f_active_whatsapp = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM conversations WHERE tenant_id = $1 AND status = 'OPEN' AND deleted_at IS NULL")
            .bind(tenant_id).fetch_one(&*p5).await
    });

    let p6 = pool.clone();
    let f_leads_month = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM leads WHERE tenant_id = $1 AND date_trunc('month', created_at) = date_trunc('month', CURRENT_TIMESTAMP) AND deleted_at IS NULL")
            .bind(tenant_id).fetch_one(&*p6).await
    });

    let p7 = pool.clone();
    let f_conversions_month = tokio::spawn(async move {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM audit_logs WHERE tenant_id = $1 AND entity_type = 'lead' AND action = 'UPDATE_LEAD' AND date_trunc('month', created_at) = date_trunc('month', CURRENT_TIMESTAMP)")
            .bind(tenant_id).fetch_one(&*p7).await
    });

    let p8 = pool.clone();
    let f_leads_status = tokio::spawn(async move {
        sqlx::query_as::<_, LeadStatusCount>("SELECT status, COUNT(*) as count FROM leads WHERE tenant_id = $1 AND deleted_at IS NULL GROUP BY status")
            .bind(tenant_id).fetch_all(&*p8).await
    });

    let p9 = pool.clone();
    let f_conv_agent = tokio::spawn(async move {
        sqlx::query_as::<_, AgentConversationCount>(
            r#"
            SELECT COALESCE(u.first_name, 'Sin Asignar') as agent_name, COUNT(c.id) as count 
            FROM conversations c 
            LEFT JOIN users u ON c.assigned_user_id = u.id 
            WHERE c.tenant_id = $1 AND c.deleted_at IS NULL 
            GROUP BY u.first_name
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&*p9)
        .await
    });

    let p10 = pool.clone();
    let f_conv_month = tokio::spawn(async move {
        sqlx::query_as::<_, MonthlyConversion>(
            r#"
            SELECT TO_CHAR(created_at, 'YYYY-MM') as month, COUNT(*) as count 
            FROM audit_logs 
            WHERE tenant_id = $1 AND entity_type = 'lead' AND action = 'UPDATE_LEAD' AND details->>'status' = 'WON'
            GROUP BY TO_CHAR(created_at, 'YYYY-MM')
            ORDER BY month ASC
            LIMIT 6
            "#
        ).bind(tenant_id).fetch_all(&*p10).await
    });

    let (
        res_total_clients,
        res_total_properties,
        res_new_leads,
        res_upcoming_appointments,
        res_active_whatsapp,
        res_leads_month,
        res_conversions_month,
        res_leads_status,
        res_conv_agent,
        res_conv_month,
    ) = tokio::join!(
        f_total_clients,
        f_total_properties,
        f_new_leads,
        f_upcoming_appointments,
        f_active_whatsapp,
        f_leads_month,
        f_conversions_month,
        f_leads_status,
        f_conv_agent,
        f_conv_month
    );

    let total_clients = res_total_clients
        .unwrap_or(Err(sqlx::Error::RowNotFound))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total_properties = res_total_properties
        .unwrap_or(Err(sqlx::Error::RowNotFound))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let new_leads = res_new_leads
        .unwrap_or(Err(sqlx::Error::RowNotFound))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let upcoming_appointments = res_upcoming_appointments
        .unwrap_or(Err(sqlx::Error::RowNotFound))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active_whatsapp_conversations = res_active_whatsapp.unwrap_or(Ok((0,))).unwrap_or((0,));
    let leads_this_month = res_leads_month.unwrap_or(Ok((0,))).unwrap_or((0,));
    let conversions_this_month = res_conversions_month.unwrap_or(Ok((0,))).unwrap_or((0,));

    let leads_by_status = res_leads_status.unwrap_or(Ok(vec![])).unwrap_or_default();
    let conversations_by_agent = res_conv_agent.unwrap_or(Ok(vec![])).unwrap_or_default();
    let conversions_by_month = res_conv_month.unwrap_or(Ok(vec![])).unwrap_or_default();

    Ok(Json(DashboardStats {
        total_clients: total_clients.0,
        total_properties: total_properties.0,
        new_leads: new_leads.0,
        upcoming_appointments: upcoming_appointments.0,
        active_whatsapp_conversations: active_whatsapp_conversations.0,
        leads_this_month: leads_this_month.0,
        conversions_this_month: conversions_this_month.0,
        leads_by_status,
        conversations_by_agent,
        conversions_by_month,
    }))
}

pub async fn get_activity(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<DashboardActivity>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    // We can fetch recent audit logs or recent leads/clients to build this.
    // Fetching the last 10 audit logs
    #[derive(sqlx::FromRow)]
    struct AuditRow {
        id: uuid::Uuid,
        action: String,
        entity_type: Option<String>,
        created_at: chrono::DateTime<chrono::Utc>,
    }

    let logs = sqlx::query_as::<_, AuditRow>(
        r#"
        SELECT id, action, entity_type, created_at
        FROM audit_logs
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let activity = logs
        .into_iter()
        .map(|log| {
            let e_type = log.entity_type.unwrap_or_else(|| "system".to_string());
            let title = match (e_type.as_str(), log.action.as_str()) {
                ("client", "CREATE_CLIENT") => "Nuevo cliente registrado".to_string(),
                ("client", "UPDATE_CLIENT") => "Cliente actualizado".to_string(),
                ("lead", "CREATE_LEAD") => "Nuevo lead recibido".to_string(),
                ("appointment", "CREATE_APPOINTMENT") => "Nueva cita agendada".to_string(),
                ("property", "CREATE_PROPERTY") => "Nueva propiedad publicada".to_string(),
                (ent, act) => format!("Acción {} en {}", act, ent),
            };

            DashboardActivity {
                id: log.id.to_string(),
                title,
                time: log.created_at.to_rfc3339(),
                r#type: e_type,
            }
        })
        .collect();

    Ok(Json(activity))
}
