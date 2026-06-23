-- Add deleted_at for Soft Delete
ALTER TABLE properties ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;
ALTER TABLE clients ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;
ALTER TABLE leads ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;
ALTER TABLE appointments ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;

-- Create indexes for deleted_at to speed up filtering
CREATE INDEX idx_properties_deleted_at ON properties(deleted_at);
CREATE INDEX idx_clients_deleted_at ON clients(deleted_at);
CREATE INDEX idx_leads_deleted_at ON leads(deleted_at);
CREATE INDEX idx_appointments_deleted_at ON appointments(deleted_at);