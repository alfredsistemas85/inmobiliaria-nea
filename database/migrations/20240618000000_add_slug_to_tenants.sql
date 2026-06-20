-- Añadir columna slug a tenants si no existe
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS slug VARCHAR(255) UNIQUE;

-- Llenar los slugs vacíos basándose en el ID para asegurar unicidad
UPDATE tenants SET slug = CONCAT('tenant-', id) WHERE slug IS NULL;

-- Asignar el slug inmonea a la Inmobiliaria Test
UPDATE tenants SET slug = 'inmonea' WHERE id = '0ec5b372-c66a-4075-9ed4-d0dd8dbfd6a8';
