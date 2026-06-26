use backend::{core, models};
use chrono::Timelike;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "worker=debug,backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Iniciando SaaS Inmobiliarias NEA Worker...");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error al conectar a PostgreSQL");

    let shared_pool = Arc::new(pool);
    tracing::info!("Worker conectado a PostgreSQL Exitosamente.");

    let adjustment_engine = Arc::new(
        backend::core::contracts::adjustment_engine::RentalAdjustmentEngine::new(
            shared_pool.clone(),
        ),
    );
    let adjustment_scheduler =
        backend::core::workers::adjustment_scheduler::RentalAdjustmentScheduler::new(
            shared_pool.clone(),
            adjustment_engine.clone(),
        );

    loop {
        tracing::debug!("Worker tick...");

        // 1. Process explicit scheduled_tasks
        match process_scheduled_tasks(shared_pool.clone()).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Processed {} explicit scheduled tasks", count);
                }
            }
            Err(e) => tracing::error!("Error processing scheduled tasks: {}", e),
        }

        // 2. Appointment Reminders (scan for upcoming appointments within next hour without confirmation)
        match process_appointment_reminders(shared_pool.clone()).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Processed {} appointment reminders", count);
                }
            }
            Err(e) => tracing::error!("Error processing appointment reminders: {}", e),
        }

        // 3. Lead Followups (scan for NUEVO leads > 24hs without followup)
        match process_lead_followups(shared_pool.clone()).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Processed {} lead followups", count);
                }
            }
            Err(e) => tracing::error!("Error processing lead followups: {}", e),
        }

        // 4. Conversation Followups (scan for unread messages > 2 hours)
        match process_conversation_followups(shared_pool.clone()).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Processed {} conversation followups", count);
                }
            }
            Err(e) => tracing::error!("Error processing conversation followups: {}", e),
        }

        // 5. Daily Rent Adjustments (run at target hour)
        let current_hour = chrono::Local::now().time().hour();
        let target_hour = std::env::var("ADJUSTMENT_SCHEDULER_HOUR")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .unwrap_or(2);

        if current_hour == target_hour {
            if let Err(e) = adjustment_scheduler.process_daily_adjustments().await {
                tracing::error!("Error processing daily rent adjustments: {:?}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

async fn process_scheduled_tasks(pool: Arc<sqlx::PgPool>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let tasks = sqlx::query!(
        r#"
        SELECT id, tenant_id, task_type, payload
        FROM scheduled_tasks
        WHERE status = 'PENDING' AND run_at <= CURRENT_TIMESTAMP
        FOR UPDATE SKIP LOCKED
        LIMIT 10
        "#
    )
    .fetch_all(&mut *tx)
    .await?;

    let count = tasks.len() as u64;

    for task in tasks {
        tracing::info!("Processing task {} (type: {})", task.id, task.task_type);
        // Mark as running
        sqlx::query!(
            "UPDATE scheduled_tasks SET status = 'RUNNING', locked_at = CURRENT_TIMESTAMP WHERE id = $1",
            task.id
        )
        .execute(&mut *tx)
        .await?;

        // --- Execute Task Logic Here based on task.task_type ---

        // Mark as completed
        sqlx::query!(
            "UPDATE scheduled_tasks SET status = 'COMPLETED', executed_at = CURRENT_TIMESTAMP WHERE id = $1",
            task.id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(count)
}

async fn process_appointment_reminders(pool: Arc<sqlx::PgPool>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Find appointments in the next 24 hours that haven't sent a confirmation
    // Note: in a real environment this uses SKIP LOCKED
    let appointments = sqlx::query!(
        r#"
        SELECT id, tenant_id, client_id, property_id, scheduled_at
        FROM appointments
        WHERE status = 'PROGRAMADA' 
          AND scheduled_at BETWEEN CURRENT_TIMESTAMP AND CURRENT_TIMESTAMP + INTERVAL '24 hours'
          AND confirmation_sent_at IS NULL
          AND deleted_at IS NULL
        FOR UPDATE SKIP LOCKED
        LIMIT 20
        "#
    )
    .fetch_all(&mut *tx)
    .await?;

    let count = appointments.len() as u64;
    let evo_client = backend::infrastructure::evolution::client::EvolutionClient::new();
    let wa_repo =
        backend::infrastructure::database::whatsapp::WhatsAppRepository::new(pool.clone());

    for appt in appointments {
        // Fetch client details
        let client = sqlx::query!(
            "SELECT first_name, phone FROM clients WHERE id = $1",
            appt.client_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Send WhatsApp
        let phone = client.phone;
        let name = client.first_name.unwrap_or_else(|| "Cliente".to_string());
        let message = format!(
            "Hola {}, te recordamos tu cita programada para el {}. Por favor confirmar.",
            name,
            appt.scheduled_at.format("%Y-%m-%d %H:%M")
        );

        if evo_client.send_message(&phone, &message).await.is_ok() {
            sqlx::query!(
                "UPDATE appointments SET confirmation_sent_at = CURRENT_TIMESTAMP WHERE id = $1",
                appt.id
            )
            .execute(&mut *tx)
            .await?;

            // Insert into history
            if let Ok(conv) = wa_repo
                .find_or_create_conversation(appt.tenant_id, appt.client_id)
                .await
            {
                let _ = wa_repo
                    .insert_message(appt.tenant_id, conv.id, None, "outbound", "bot", &message)
                    .await;
            }
        }
    }

    tx.commit().await?;
    Ok(count)
}

async fn process_lead_followups(pool: Arc<sqlx::PgPool>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let leads = sqlx::query!(
        r#"
        SELECT id, tenant_id, client_id, assigned_to
        FROM leads
        WHERE status = 'NUEVO' 
          AND created_at <= CURRENT_TIMESTAMP - INTERVAL '24 hours'
          AND updated_at <= CURRENT_TIMESTAMP - INTERVAL '24 hours'
          AND deleted_at IS NULL
        FOR UPDATE SKIP LOCKED
        LIMIT 20
        "#
    )
    .fetch_all(&mut *tx)
    .await?;

    let count = leads.len() as u64;
    let notif_repo =
        backend::infrastructure::database::notifications::NotificationRepository::new(pool.clone());

    for lead in leads {
        // Notify the assigned agent or admin
        let _ = notif_repo.create(
            lead.tenant_id,
            lead.assigned_to,
            "LEAD_FOLLOWUP",
            "Recordatorio de Lead",
            "Tienes un lead en estado NUEVO por más de 24 horas. ¡Es hora de hacer seguimiento!",
        ).await;

        // INC-025: Instead of changing the status to 'CONTACTADO', we update 'updated_at'
        // to avoid spamming the reminder every 10 seconds, pushing the next reminder 24hs later
        // unless the status is manually changed.
        sqlx::query!(
            "UPDATE leads SET updated_at = CURRENT_TIMESTAMP WHERE id = $1",
            lead.id
        )
        .execute(&mut *tx)
        .await?;

        // Create a generic audit log.
        sqlx::query!(
            "INSERT INTO audit_logs (tenant_id, action, entity_type, entity_id, details) VALUES ($1, 'LEAD_FOLLOWUP_SENT', 'lead', $2, '{\"reason\":\"> 24hs NUEVO\"}')",
            lead.tenant_id, lead.id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(count)
}

async fn process_conversation_followups(pool: Arc<sqlx::PgPool>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Find conversations with unread messages and last message older than 2 hours
    let conversations = sqlx::query!(
        r#"
        SELECT id, tenant_id, client_id, assigned_user_id
        FROM conversations
        WHERE status = 'OPEN' 
          AND unread_count > 0
          AND last_message_at <= CURRENT_TIMESTAMP - INTERVAL '2 hours'
          AND deleted_at IS NULL
        FOR UPDATE SKIP LOCKED
        LIMIT 20
        "#
    )
    .fetch_all(&mut *tx)
    .await?;

    let count = conversations.len() as u64;
    let notif_repo =
        backend::infrastructure::database::notifications::NotificationRepository::new(pool.clone());

    for conv in conversations {
        if let Some(user_id) = conv.assigned_user_id {
            // Notify the assigned agent
            let _ = notif_repo.create(
                conv.tenant_id,
                Some(user_id),
                "CONVERSATION_FOLLOWUP",
                "Recordatorio de Conversación",
                "Tienes mensajes sin leer por más de 2 horas en una conversación. ¡Es hora de responder!",
            ).await;
        }

        // Create a generic audit log.
        sqlx::query!(
            "INSERT INTO audit_logs (tenant_id, action, entity_type, entity_id, details) VALUES ($1, 'CONVERSATION_FOLLOWUP_SENT', 'conversation', $2, '{\"reason\":\"> 2hs UNREAD\"}')",
            conv.tenant_id, conv.id
        )
        .execute(&mut *tx)
        .await?;

        // We will reset unread_count so it doesn't spam
        sqlx::query!(
            "UPDATE conversations SET unread_count = 0 WHERE id = $1",
            conv.id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(count)
}
