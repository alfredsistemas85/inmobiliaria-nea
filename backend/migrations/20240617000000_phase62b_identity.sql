-- Modificaciones a la tabla users
ALTER TABLE users ADD COLUMN email_verified_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE users ADD COLUMN verification_token VARCHAR(255) DEFAULT NULL;
ALTER TABLE users ADD COLUMN verification_sent_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE users ADD COLUMN email_type VARCHAR(50) DEFAULT 'PUBLIC';

-- Actualizar usuarios existentes para considerarlos verificados
UPDATE users SET email_verified_at = CURRENT_TIMESTAMP WHERE email_verified_at IS NULL;

-- Modificaciones a la tabla tenants
ALTER TABLE tenants ADD COLUMN status VARCHAR(50) DEFAULT 'APPROVED';

-- Actualizar tenants existentes para considerarlos aprobados
UPDATE tenants SET status = 'APPROVED' WHERE status IS NULL;

-- Indice para búsquedas rápidas de tokens
CREATE INDEX idx_users_verification_token ON users(verification_token);
