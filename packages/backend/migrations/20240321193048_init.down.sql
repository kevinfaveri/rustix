-- Add down migration script here
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS trippplanbox;
DROP TABLE IF EXISTS users;
DROP EXTENSION IF EXISTS "uuid-ossp";
