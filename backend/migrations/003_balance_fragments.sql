-- ============================================
-- Migración 003: Sistema de Fragmentos de Balance
-- Ejecutar en Supabase SQL Editor
-- ============================================
-- Este cambio modifica la lógica de balances:
-- - El Leader crea el balance y selecciona asignaturas
-- - Cada SubjectLeader llena su fragmento correspondiente
-- ============================================

-- ============================================
-- PASO 1: Modificar tabla balances
-- ============================================

-- Agregar nuevas columnas a balances
ALTER TABLE balances 
ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'draft',
ADD COLUMN IF NOT EXISTS deadline DATE DEFAULT NULL,
ADD COLUMN IF NOT EXISTS allow_leader_edit BOOLEAN NOT NULL DEFAULT false;

-- Comentario: status puede ser 'draft', 'in_progress', 'completed'
-- deadline: fecha límite para que los SubjectLeaders completen sus fragmentos
-- allow_leader_edit: si el Leader puede editar fragmentos de SubjectLeaders

-- Eliminar la columna subjects (ya no se usa, los datos van en fragments)
-- NOTA: Hacer backup primero si hay datos importantes
-- ALTER TABLE balances DROP COLUMN IF EXISTS subjects;

-- Por ahora la dejamos para no perder datos existentes, pero no se usará
-- En producción, migrar datos existentes antes de eliminar

-- ============================================
-- PASO 2: Crear tabla balance_fragments
-- ============================================

CREATE TABLE IF NOT EXISTS balance_fragments (
    id SERIAL PRIMARY KEY,
    
    -- Relaciones
    balance_id INTEGER NOT NULL REFERENCES balances(id) ON DELETE CASCADE,
    asignatura_id INTEGER NOT NULL REFERENCES asignaturas(id) ON DELETE CASCADE,
    subject_leader_id INTEGER REFERENCES usuarios(id) ON DELETE SET NULL,
    
    -- Estado del fragmento
    status TEXT NOT NULL DEFAULT 'pending',  -- 'pending', 'in_progress', 'completed'
    
    -- Datos del fragmento (distribución semanal de la asignatura)
    -- Estructura: { "values": ["C", "CP", "", ...], "totals": {...}, "notes": "..." }
    data JSONB NOT NULL DEFAULT '{}',
    
    -- Timestamps
    completed_at TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Restricción: una asignatura solo puede aparecer una vez por balance
    UNIQUE(balance_id, asignatura_id)
);

-- ============================================
-- PASO 3: Índices para optimización
-- ============================================

-- Índice para buscar fragmentos por balance
CREATE INDEX IF NOT EXISTS idx_balance_fragments_balance_id 
ON balance_fragments(balance_id);

-- Índice para buscar fragmentos por asignatura
CREATE INDEX IF NOT EXISTS idx_balance_fragments_asignatura_id 
ON balance_fragments(asignatura_id);

-- Índice para buscar fragmentos por subject_leader (para dashboard)
CREATE INDEX IF NOT EXISTS idx_balance_fragments_subject_leader_id 
ON balance_fragments(subject_leader_id);

-- Índice para buscar fragmentos pendientes
CREATE INDEX IF NOT EXISTS idx_balance_fragments_status 
ON balance_fragments(status);

-- Índice compuesto para consultas de SubjectLeader
CREATE INDEX IF NOT EXISTS idx_balance_fragments_leader_status 
ON balance_fragments(subject_leader_id, status);

-- ============================================
-- PASO 4: Función para actualizar updated_at automáticamente
-- ============================================

-- Función trigger para actualizar updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger para balance_fragments
DROP TRIGGER IF EXISTS update_balance_fragments_updated_at ON balance_fragments;
CREATE TRIGGER update_balance_fragments_updated_at
    BEFORE UPDATE ON balance_fragments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger para balances (si no existe)
DROP TRIGGER IF EXISTS update_balances_updated_at ON balances;
CREATE TRIGGER update_balances_updated_at
    BEFORE UPDATE ON balances
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- PASO 5: Registrar migración
-- ============================================

INSERT INTO schema_migrations (version, description)
VALUES ('003', 'Add balance_fragments table for modular balance editing by SubjectLeaders')
ON CONFLICT (version) DO NOTHING;

-- ============================================
-- Verificación
-- ============================================

SELECT 'Migración 003 aplicada correctamente' as status;

-- Verificar estructura de tablas
SELECT column_name, data_type, is_nullable, column_default
FROM information_schema.columns 
WHERE table_name = 'balance_fragments'
ORDER BY ordinal_position;

