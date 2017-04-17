-- Your SQL goes here
CREATE TABLE sessions (
    id VARCHAR(24) PRIMARY KEY,
    user_id INTEGER NOT NULL
) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_520_ci;