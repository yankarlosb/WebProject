-- ============================================
-- Migración 004: Períodos No Académicos en Balances
-- Ejecutar en Supabase SQL Editor
-- ============================================
-- Permite almacenar fechas de vacaciones, días feriados, etc.
-- que se excluyen del cálculo de semanas académicas
-- ============================================

-- Agregar columna para períodos no académicos
-- Estructura: [{ "start": "2026-01-15", "end": "2026-01-20", "name": "Vacaciones" }, ...]
ALTER TABLE balances 
ADD COLUMN IF NOT EXISTS non_academic_periods JSONB NOT NULL DEFAULT '[]';

-- Comentario: 
-- Esta columna almacena intervalos de fechas que NO son académicas
-- El frontend usará esta información para:
-- 1. Calcular las fechas reales de cada semana (saltando vacaciones)
-- 2. Mostrar "S1: 12/01 - 16/01" en las tablas de balance
-- Los fines de semana se excluyen automáticamente en el cálculo

-- Registrar migración
INSERT INTO schema_migrations (version, description)
VALUES ('004', 'Add non_academic_periods JSONB column to balances')
ON CONFLICT (version) DO NOTHING;

-- Verificar
SELECT 'Columna non_academic_periods añadida a balances' as status;
