-- Crear la tabla machinery
CREATE TABLE machinery (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    manufacturer TEXT,
    model TEXT,
    serial_number TEXT,
    purchase_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status TEXT,
    location TEXT,
    cost NUMERIC,
    notes TEXT
);

-- Crear la función para generar el ID personalizado
CREATE OR REPLACE FUNCTION generate_machinery_id() RETURNS TRIGGER AS $$
DECLARE
    new_id TEXT;
BEGIN
    -- Generar el nuevo ID basado en el último ID existente
    SELECT 'MAC' || LPAD((COALESCE(MAX(SUBSTRING(id FROM 3)::INTEGER), 0) + 1)::TEXT, 3, '0')
    INTO new_id
    FROM machinery;

    -- Asignar el nuevo ID al registro
    NEW.id := new_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Crear el trigger que usa la función anterior
CREATE TRIGGER set_machinery_id
    BEFORE INSERT ON machinery
    FOR EACH ROW
    WHEN (NEW.id IS NULL)
EXECUTE FUNCTION generate_machinery_id();

-- Ejemplo de cómo insertar un nuevo registro
INSERT INTO machinery (name, manufacturer, model, serial_number, purchase_date, status, location, cost, notes)
VALUES ('Excavator', 'Caterpillar', '320D', 'CAT0320D12345', '2023-10-01', 'Operational', 'Site A', 150000, 'Regular maintenance required');