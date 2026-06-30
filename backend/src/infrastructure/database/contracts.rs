use crate::api::contracts::dto::CreateContractDtoV2;
use crate::api::contracts::models::Contract;
use chrono::Datelike;
use sqlx::{Connection, PgPool};
use std::sync::Arc;
use uuid::Uuid;

pub struct ContractRepository {
    pool: Arc<PgPool>,
}

impl ContractRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, payload), fields(tenant_id = %tenant_id, user_id = %user_id))]
    pub async fn create_contract_v2(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        payload: CreateContractDtoV2,
    ) -> Result<Contract, String> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            e.to_string()
        })?;

        let contract = sqlx::query_as::<_, Contract>(
            r#"
            INSERT INTO contracts (
                tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, rent_amount, 
                adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days,
                c_type, c_destination, jurisdiction, city, province, currency, deposit_amount, commission_amount, fees_amount,
                taxes_payer, services_payer, observations, status, template_id, created_by, updated_by, parent_contract_id
            )
            VALUES ($1, $2, $3, $4, $5, $5, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23::contract_status, $24, $25, $25, $26)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(payload.property_id)
        .bind(payload.start_date)
        .bind(payload.end_date)
        .bind(payload.original_rent_amount)
        .bind(payload.adjustment_method.clone())
        .bind(payload.adjustment_frequency.clone())
        .bind(payload.automation_mode.clone())
        .bind(payload.fixed_percentage)
        .bind(payload.notification_days_before)
        .bind(payload.c_type.clone())
        .bind(payload.c_destination.clone())
        .bind(payload.jurisdiction.clone())
        .bind(payload.city.clone())
        .bind(payload.province.clone())
        .bind(payload.currency.clone())
        .bind(payload.deposit_amount)
        .bind(payload.commission_amount)
        .bind(payload.fees_amount)
        .bind(payload.taxes_payer.clone())
        .bind(payload.services_payer.clone())
        .bind(payload.observations.clone())
        .bind(payload.status.as_deref().unwrap_or("ACTIVE"))
        .bind(payload.template_id)
        .bind(user_id)
        .bind(payload.parent_contract_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("23P01") {
                    return "HTTP 409: La propiedad ya posee un contrato activo para ese período.".to_string();
                }
            }
            tracing::error!("Error insertando contrato V2 en BD: {:?}", e);
            format!("Error creando contrato: {:?}", e)
        })?;

        for participant_dto in payload.participants {
            let participant_id = uuid::Uuid::new_v4();

            sqlx::query(
                r#"
                INSERT INTO contract_participants (
                    id, tenant_id, contract_id, client_id, p_role, percentage, is_main, display_order, observations, created_by, updated_by
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
                "#
            )
            .bind(participant_id)
            .bind(tenant_id)
            .bind(contract.id)
            .bind(participant_dto.client_id)
            .bind(participant_dto.p_role.clone())
            .bind(participant_dto.percentage)
            .bind(participant_dto.is_main)
            .bind(participant_dto.display_order)
            .bind(participant_dto.observations.clone())
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Error insertando participante: {:?}", e);
                format!("Error insertando participante: {:?}", e)
            })?;

            if let Some(guarantees) = participant_dto.guarantees {
                for guarantee_dto in guarantees {
                    sqlx::query(
                        r#"
                        INSERT INTO participant_guarantees (
                            id, tenant_id, participant_id, guarantee_type, income_amount, employer, guarantee_details, observations, created_by, updated_by
                        )
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
                        "#
                    )
                    .bind(uuid::Uuid::new_v4())
                    .bind(tenant_id)
                    .bind(participant_id)
                    .bind(guarantee_dto.guarantee_type.clone())
                    .bind(guarantee_dto.income_amount)
                    .bind(guarantee_dto.employer.clone())
                    .bind(guarantee_dto.guarantee_details.clone())
                    .bind(guarantee_dto.observations.clone())
                    .bind(user_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        tracing::error!("Error insertando garantia: {}", e);
                        "Error insertando garantia".to_string()
                    })?;
                }
            }
        }

        // Insert terms if present
        if let Some(terms) = payload.terms {
            sqlx::query(
                r#"
                INSERT INTO contract_terms (
                    tenant_id, contract_id, allows_pets, allows_sublease, requires_inventory, requires_insurance, automatic_renewal,
                    permitted_activity, notice_days, early_termination_penalty, observations, created_by, updated_by
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12)
                "#
            )
            .bind(tenant_id)
            .bind(contract.id)
            .bind(terms.allows_pets.unwrap_or(false))
            .bind(terms.allows_sublease.unwrap_or(false))
            .bind(terms.requires_inventory.unwrap_or(false))
            .bind(terms.requires_insurance.unwrap_or(false))
            .bind(terms.automatic_renewal.unwrap_or(false))
            .bind(terms.permitted_activity)
            .bind(terms.notice_days)
            .bind(terms.early_termination_penalty)
            .bind(terms.observations)
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Error insertando contract_terms: {}", e);
                "Error insertando terminos del contrato".to_string()
            })?;
        }

        // Insert clauses if present
        if let Some(clauses) = payload.clauses {
            for clause in clauses {
                sqlx::query(
                    r#"
                    INSERT INTO contract_clauses (
                        tenant_id, contract_id, display_order, title, body, is_mandatory, is_editable, is_system, created_by, updated_by
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
                    "#
                )
                .bind(tenant_id)
                .bind(contract.id)
                .bind(clause.display_order)
                .bind(clause.title)
                .bind(clause.body)
                .bind(clause.is_mandatory.unwrap_or(false))
                .bind(clause.is_editable.unwrap_or(true))
                .bind(clause.is_system.unwrap_or(false))
                .bind(user_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    tracing::error!("Error insertando contract_clause: {:?}", e);
                    format!("Error insertando clausula: {:?}", e)
                })?;
            }
        }

        if payload.status.as_deref().unwrap_or("ACTIVE") != "DRAFT" {
            let mut current_date = payload.start_date;
            while current_date < payload.end_date {
            let mut year = current_date.year();
            let mut month = current_date.month();

            let due_day = if payload.start_date.day() > 10 {
                payload.start_date.day()
            } else {
                10
            };
            let due_date =
                chrono::NaiveDate::from_ymd_opt(year, month, due_day).unwrap_or(current_date);

            sqlx::query(
                "INSERT INTO contract_installments (id, tenant_id, contract_id, due_date, amount, status) VALUES ($1, $2, $3, $4, $5, 'PENDING')"
            )
            .bind(uuid::Uuid::new_v4())
            .bind(tenant_id)
            .bind(contract.id)
            .bind(due_date)
            .bind(payload.original_rent_amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Error creating installment: {}", e);
                "Error creando cuota".to_string()
            })?;

            if month == 12 {
                year += 1;
                month = 1;
            } else {
                month += 1;
            }

            let next_day = if current_date.day() > 28 {
                28
            } else {
                current_date.day()
            };
                current_date = chrono::NaiveDate::from_ymd_opt(year, month, next_day)
                    .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap());
            }
        }

        tx.commit()
            .await
            .map_err(|_| "Error en commit".to_string())?;

        Ok(contract)
    }

    pub async fn get_contract(
        &self,
        tenant_id: Uuid,
        contract_id: Uuid,
    ) -> Result<serde_json::Value, String> {
        // Query utilizing JOIN and jsonb_agg to avoid N+1 queries.
        let row: (serde_json::Value,) = sqlx::query_as(
            r#"
            SELECT json_build_object(
                'contract', row_to_json(c.*),
                'property_address', (SELECT COALESCE(street, title) FROM properties p WHERE p.id = c.property_id),
                'terms', (SELECT row_to_json(ct.*) FROM contract_terms ct WHERE ct.contract_id = c.id LIMIT 1),
                'participants', (
                    SELECT COALESCE(json_agg(
                        jsonb_build_object(
                            'id', cp.id,
                            'client_id', cp.client_id,
                            'client_name', (SELECT CONCAT(first_name, ' ', last_name) FROM clients cl WHERE cl.id = cp.client_id),
                            'p_role', cp.p_role,
                            'percentage', cp.percentage,
                            'is_main', cp.is_main,
                            'guarantees', (
                                SELECT COALESCE(json_agg(row_to_json(g.*)), '[]'::json)
                                FROM participant_guarantees g
                                WHERE g.participant_id = cp.id
                            )
                        )
                    ), '[]'::json)
                    FROM contract_participants cp
                    WHERE cp.contract_id = c.id
                ),
                'clauses', (
                    SELECT COALESCE(json_agg(row_to_json(cc.*) ORDER BY cc.display_order ASC), '[]'::json)
                    FROM contract_clauses cc
                    WHERE cc.contract_id = c.id
                )
            )
            FROM contracts c
            WHERE c.id = $1 AND c.tenant_id = $2
            "#
        )
        .bind(contract_id)
        .bind(tenant_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.0)
    }
}
