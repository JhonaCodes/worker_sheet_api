CREATE TABLE IF NOT EXISTS workers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    position VARCHAR(100) NOT NULL,
    department VARCHAR(100),
    salary DECIMAL(10,2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insertar algunos datos de ejemplo
INSERT INTO workers (name, position, department, salary) VALUES
    ('Juan Pérez', 'Desarrollador Senior', 'Tecnología', 75000.00),
    ('Ana López', 'Project Manager', 'Gestión', 85000.00),
    ('Carlos Ruiz', 'Desarrollador Junior', 'Tecnología', 45000.00);