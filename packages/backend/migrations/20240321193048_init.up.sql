-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    user_name VARCHAR(255),
    user_avatar VARCHAR(2048),
    user_prompt_persona VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create TripplanBox table
CREATE TABLE trippplanbox (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    user_id UUID NOT NULL,
    booked BOOLEAN NOT NULL,
    published BOOLEAN NOT NULL,
    published_link VARCHAR(255),
    view_mode VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Create Messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content JSONB NOT NULL,
    layout_content JSONB NOT NULL,
    trippplanbox_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (trippplanbox_id) REFERENCES trippplanbox(id)
);

CREATE TABLE "user_sessions" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "user_id" UUID NOT NULL,
    "session_token_p1" text NOT NULL,
    "session_token_p2" text NOT NULL,
    "oauth_provider" text NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE "oauth2_state_storage" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "csrf_state" text NOT NULL,
    "pkce_code_verifier" text NOT NULL
);