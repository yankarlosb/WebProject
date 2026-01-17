-- Migration: Add must_change_password column to users table
ALTER TABLE usuarios ADD COLUMN IF NOT EXISTS must_change_password BOOLEAN NOT NULL DEFAULT TRUE;
