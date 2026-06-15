ALTER TABLE tenants
ADD COLUMN slug VARCHAR(120);

CREATE UNIQUE INDEX idx_tenants_slug
ON tenants(slug);
