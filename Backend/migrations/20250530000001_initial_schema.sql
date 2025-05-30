
-- SecureShareSystem Database Schema
BEGIN;
-- Drop existing tables if they exist (in reverse order due to foreign keys)
DROP TABLE IF EXISTS shared_links CASCADE;
DROP TABLE IF EXISTS images CASCADE;
DROP TABLE IF EXISTS users CASCADE;


-- Users table for authentication
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL, -- Argon2 hashed password
    rsa_public_key TEXT, -- RSA public key for encryption (match your Rust model)
    rsa_public_key_length INTEGER DEFAULT 2048, -- Match your Rust model
    api_token VARCHAR(255), -- Match your Rust model
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- Images table for storing encrypted image metadata
CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(50) NOT NULL,
    image_location VARCHAR(500) NOT NULL, -- Match your Rust model (was encrypted_path)
    aes_encryption_key_encrypted TEXT NOT NULL, -- Match your Rust model
    file_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity
    iv VARCHAR(32) NOT NULL, -- Initialization vector for AES
    upload_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expiration_time TIMESTAMP,
    is_public BOOLEAN DEFAULT FALSE
);

-- Shared links table for managing image sharing
CREATE TABLE IF NOT EXISTS shared_links (
    id SERIAL PRIMARY KEY,
    image_id INTEGER REFERENCES images(id) ON DELETE CASCADE,
    share_token VARCHAR(255) UNIQUE NOT NULL, -- UUID for sharing
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    max_views INTEGER DEFAULT NULL,
    current_views INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_images_user_id ON images(user_id);
CREATE INDEX IF NOT EXISTS idx_images_upload_time ON images(upload_time);
CREATE INDEX IF NOT EXISTS idx_shared_links_token ON shared_links(share_token);
CREATE INDEX IF NOT EXISTS idx_shared_links_image_id ON shared_links(image_id);

COMMIT;
