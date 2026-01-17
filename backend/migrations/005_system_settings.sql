-- Migration: 005_system_settings.sql
-- Description: Add system_settings table for configurable system parameters
-- Date: 2026-01-11

-- System settings table for runtime-configurable parameters
CREATE TABLE IF NOT EXISTS system_settings (
    key VARCHAR(100) PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL DEFAULT 'general',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for category-based queries
CREATE INDEX IF NOT EXISTS idx_system_settings_category ON system_settings(category);

-- Insert default settings
INSERT INTO system_settings (key, value, description, category) VALUES
    -- Security settings
    ('max_login_attempts', '5', 'Máximo de intentos de login fallidos antes del bloqueo', 'security'),
    ('block_duration_minutes', '5', 'Duración del bloqueo por intentos fallidos (minutos)', 'security'),
    ('attempt_window_minutes', '1', 'Ventana de tiempo para contar intentos fallidos (minutos)', 'security'),
    ('token_expiration_hours', '3', 'Duración del token JWT (horas)', 'security'),
    ('require_ip_validation', 'true', 'Validar IP en cada solicitud con el token', 'security'),
    
    -- Session settings
    ('session_timeout_minutes', '30', 'Tiempo de inactividad antes de cerrar sesión (minutos)', 'session'),
    ('max_concurrent_sessions', '3', 'Máximo de sesiones concurrentes por usuario', 'session'),
    
    -- Password policy
    ('password_min_length', '8', 'Longitud mínima de contraseña', 'password'),
    ('password_require_uppercase', 'true', 'Requerir mayúsculas en contraseña', 'password'),
    ('password_require_lowercase', 'true', 'Requerir minúsculas en contraseña', 'password'),
    ('password_require_special', 'true', 'Requerir caracteres especiales en contraseña', 'password'),
    
    -- Audit settings
    ('audit_retention_days', '90', 'Días de retención de logs de auditoría', 'audit'),
    ('audit_log_ip', 'true', 'Registrar dirección IP en logs', 'audit')
ON CONFLICT (key) DO NOTHING;

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_system_settings_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_system_settings_timestamp ON system_settings;
CREATE TRIGGER trigger_update_system_settings_timestamp
    BEFORE UPDATE ON system_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_system_settings_timestamp();

-- Record migration
INSERT INTO schema_migrations (version, name, applied_at)
VALUES (5, '005_system_settings', CURRENT_TIMESTAMP)
ON CONFLICT (version) DO NOTHING;
