-- ============================================
-- Schema Base - Balance de Carga Docente
-- ============================================
-- NOTA: Para cambios incrementales, usa el sistema de migraciones
-- en el directorio migrations/
-- Aplicar migraciones: cd migrations && ./apply_all.fish
-- ============================================

-- Tabla de control de migraciones
CREATE TABLE IF NOT EXISTS schema_migrations (
    id SERIAL PRIMARY KEY,
    version TEXT NOT NULL UNIQUE,
    description TEXT,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de usuarios
CREATE TABLE IF NOT EXISTS usuarios (
    id SERIAL PRIMARY KEY,
    user_name TEXT NOT NULL, 
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    token TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    role TEXT DEFAULT 'user'
);

-- Tabla de asignaturas
CREATE TABLE IF NOT EXISTS asignaturas (
    id SERIAL PRIMARY KEY,
    leader_id INTEGER UNIQUE REFERENCES usuarios(id) DEFAULT NULL,
    name TEXT NOT NULL,
    year TEXT NOT NULL,
    semester TEXT NOT NULL,
    C INTEGER DEFAULT NULL, 
    CP INTEGER DEFAULT NULL,
    S INTEGER DEFAULT NULL,
    PL INTEGER DEFAULT NULL,
    TE INTEGER DEFAULT NULL,
    T INTEGER DEFAULT NULL,
    PP INTEGER DEFAULT NULL,
    EC INTEGER DEFAULT NULL,
    TC INTEGER DEFAULT NULL,
    EF INTEGER DEFAULT NULL,
    hours INTEGER NOT NULL,
    date_start TIMESTAMP NOT NULL,
    date_end TIMESTAMP NOT NULL
);

-- Tabla de balances de carga docente
-- La columna 'subjects' almacena un array JSON con las asignaturas y sus valores
-- Estructura: [{ "id": "string", "name": "string", "values": ["C", "CP", "", ...] }, ...]
CREATE TABLE IF NOT EXISTS balances (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    name TEXT NOT NULL DEFAULT 'Balance sin nombre',
    academic_year TEXT NOT NULL,           -- '1ro', '2do', '3ro', '4to'
    period TEXT NOT NULL,                  -- '1ero', '2do'
    academic_year_text TEXT NOT NULL,      -- '2025-2026'
    start_date DATE NOT NULL,
    weeks INTEGER NOT NULL DEFAULT 15,     -- Número de semanas (variable)
    subjects JSONB NOT NULL DEFAULT '[]',  -- Array de asignaturas con valores
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índices para optimizar queries de balances
CREATE INDEX IF NOT EXISTS idx_balances_user_id ON balances(user_id);
CREATE INDEX IF NOT EXISTS idx_balances_created ON balances(created_at DESC);

-- Registrar schema inicial
INSERT INTO schema_migrations (version, description) 
VALUES ('000', 'Initial schema - usuarios and asignaturas tables')
ON CONFLICT (version) DO NOTHING;

-- Registrar migración de balances
INSERT INTO schema_migrations (version, description)
VALUES ('001', 'Add balances table with JSONB subjects')
ON CONFLICT (version) DO NOTHING;
