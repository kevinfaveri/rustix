-- Add down migration script here
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS trippplanbox;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS "user_sessions";
DROP TABLE IF EXISTS "oauth2_state_storage";
DROP EXTENSION IF EXISTS "uuid-ossp";
