-- ============================================
-- Migración 001: Tabla de Balances
-- Ejecutar en Supabase SQL Editor
-- ============================================

-- Eliminar tabla balance antigua si existe (era una versión incompleta)
DROP TABLE IF EXISTS balance CASCADE;

-- Crear tabla de balances de carga docente
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

-- Índices para optimizar queries
CREATE INDEX IF NOT EXISTS idx_balances_user_id ON balances(user_id);
CREATE INDEX IF NOT EXISTS idx_balances_created ON balances(created_at DESC);

-- Registrar migración
INSERT INTO schema_migrations (version, description)
VALUES ('001', 'Add balances table with JSONB subjects')
ON CONFLICT (version) DO NOTHING;

-- Verificar creación
SELECT 'Tabla balances creada correctamente' as status;
