-- Add missing indexes for Foreign Keys to prevent Sequential Scans on joins and deletes.

-- subscriptions
CREATE INDEX IF NOT EXISTS idx_subscriptions_plan_id ON subscriptions(plan_id);

-- property_images
CREATE INDEX IF NOT EXISTS idx_property_images_tenant_id ON property_images(tenant_id);
CREATE INDEX IF NOT EXISTS idx_property_images_property_id ON property_images(property_id);

-- property_documents
CREATE INDEX IF NOT EXISTS idx_property_documents_tenant_id ON property_documents(tenant_id);
CREATE INDEX IF NOT EXISTS idx_property_documents_property_id ON property_documents(property_id);

-- leads
CREATE INDEX IF NOT EXISTS idx_leads_client_id ON leads(client_id);
CREATE INDEX IF NOT EXISTS idx_leads_property_id ON leads(property_id);
CREATE INDEX IF NOT EXISTS idx_leads_assigned_to ON leads(assigned_to);

-- lead_activities
CREATE INDEX IF NOT EXISTS idx_lead_activities_tenant_id ON lead_activities(tenant_id);
CREATE INDEX IF NOT EXISTS idx_lead_activities_lead_id ON lead_activities(lead_id);
CREATE INDEX IF NOT EXISTS idx_lead_activities_user_id ON lead_activities(user_id);

-- appointments
CREATE INDEX IF NOT EXISTS idx_appointments_client_id ON appointments(client_id);
CREATE INDEX IF NOT EXISTS idx_appointments_property_id ON appointments(property_id);
CREATE INDEX IF NOT EXISTS idx_appointments_user_id ON appointments(user_id);

-- appointment_reminders
CREATE INDEX IF NOT EXISTS idx_appointment_reminders_tenant_id ON appointment_reminders(tenant_id);
CREATE INDEX IF NOT EXISTS idx_appointment_reminders_appointment_id ON appointment_reminders(appointment_id);

-- conversations
CREATE INDEX IF NOT EXISTS idx_conversations_client_id ON conversations(client_id);

-- ai_prompts
CREATE INDEX IF NOT EXISTS idx_ai_prompts_tenant_id ON ai_prompts(tenant_id);

-- audit_logs
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);

-- appointment_audit_logs
CREATE INDEX IF NOT EXISTS idx_appointment_audit_logs_tenant_id ON appointment_audit_logs(tenant_id);

-- scheduled_tasks
CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_tenant_id ON scheduled_tasks(tenant_id);

-- calendar_sync_events
CREATE INDEX IF NOT EXISTS idx_calendar_sync_events_user_id ON calendar_sync_events(user_id);

-- documents
CREATE INDEX IF NOT EXISTS idx_documents_uploaded_by ON documents(uploaded_by);
CREATE INDEX IF NOT EXISTS idx_documents_rent_adjustment_id ON documents(rent_adjustment_id);

-- document_access_logs
CREATE INDEX IF NOT EXISTS idx_document_access_logs_tenant_id ON document_access_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_document_access_logs_user_id ON document_access_logs(user_id);

-- digital_signatures
CREATE INDEX IF NOT EXISTS idx_digital_signatures_document_id ON digital_signatures(document_id);

-- system_errors
CREATE INDEX IF NOT EXISTS idx_system_errors_resolved_by ON system_errors(resolved_by);

-- support_tickets
CREATE INDEX IF NOT EXISTS idx_support_tickets_user_id ON support_tickets(user_id);

-- support_ticket_messages
CREATE INDEX IF NOT EXISTS idx_support_ticket_messages_sender_id ON support_ticket_messages(sender_id);

-- contracts
CREATE INDEX IF NOT EXISTS idx_contracts_tenant_user_id ON contracts(tenant_user_id);
CREATE INDEX IF NOT EXISTS idx_contracts_client_id ON contracts(client_id);

-- invoices
CREATE INDEX IF NOT EXISTS idx_invoices_contract_id ON invoices(contract_id);

-- payments
CREATE INDEX IF NOT EXISTS idx_payments_invoice_id ON payments(invoice_id);

-- rent_adjustments
CREATE INDEX IF NOT EXISTS idx_rent_adjustments_approved_by ON rent_adjustments(approved_by);

-- properties
CREATE INDEX IF NOT EXISTS idx_properties_owner_id ON properties(owner_id);
