-- Migration: Add audit_logs table for security and functional auditing
-- Version: 002
-- Description: Creates audit_logs table to track all system events

CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    -- User information
    user_id INTEGER REFERENCES usuarios(id) ON DELETE SET NULL,
    user_name TEXT,                           -- Stored separately in case user is deleted
    
    -- Event information
    event_type TEXT NOT NULL,                 -- LOGIN, LOGOUT, CREATE, UPDATE, DELETE, ERROR, ACCESS_DENIED
    category TEXT NOT NULL,                   -- SECURITY, FUNCTIONAL
    
    -- Details
    entity_type TEXT,                         -- USER, SUBJECT, BALANCE, SESSION
    entity_id INTEGER,                        -- ID of affected entity
    description TEXT NOT NULL,                -- Human-readable description
    
    -- Context
    ip_address TEXT,                          -- Client IP
    user_agent TEXT,                          -- Browser/client info
    
    -- Result
    success BOOLEAN NOT NULL DEFAULT true,    -- Whether the operation succeeded
    error_message TEXT,                       -- Error details if failed
    
    -- Timestamp
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX IF NOT EXISTS idx_audit_logs_category ON audit_logs(category);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity ON audit_logs(entity_type, entity_id);

-- Register migration
INSERT INTO schema_migrations (version, description)
VALUES ('002', 'Add audit_logs table for security and functional auditing')
ON CONFLICT (version) DO NOTHING;
