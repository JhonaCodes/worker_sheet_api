-- Crear la tabla components
CREATE TABLE components (
    id TEXT PRIMARY KEY,
    machinery_id TEXT NOT NULL,
    parent_id TEXT,
    name TEXT NOT NULL,
    serial_number TEXT,
    status TEXT,
    location TEXT,
    cost NUMERIC,
    notes TEXT,
    FOREIGN KEY (machinery_id) REFERENCES machinery(id),
    FOREIGN KEY (parent_id) REFERENCES components(id)
);

-- Crear la función para generar el ID personalizado para components
CREATE OR REPLACE FUNCTION generate_component_id() RETURNS TRIGGER AS $$
DECLARE
    new_id TEXT;
BEGIN
    -- Generar el nuevo ID basado en el último ID existente
    SELECT 'COM' || LPAD((COALESCE(MAX(SUBSTRING(id FROM 4)::INTEGER), 0) + 1)::TEXT, 3, '0')
    INTO new_id
    FROM components;

    -- Asignar el nuevo ID al registro
    NEW.id := new_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Crear el trigger que usa la función anterior
CREATE TRIGGER set_component_id
    BEFORE INSERT ON components
    FOR EACH ROW
    WHEN (NEW.id IS NULL)
EXECUTE FUNCTION generate_component_id();