-- Add up migration script here
CREATE TABLE IF NOT EXISTS jokes(
id INTEGER PRIMARY KEY AUTOINCREMENT,
author TEXT,
value TEXT,
created_at NUMERIC,
updated_at NUMERIC);
