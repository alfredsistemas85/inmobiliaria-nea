CREATE TABLE IF NOT EXISTS system_settings (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default SaaS subscription price
INSERT INTO system_settings (key, value) 
VALUES ('SAAS_SUBSCRIPTION_PRICE', '50000')
ON CONFLICT (key) DO NOTHING;
