-- Users

CREATE TABLE users
(
    id            uuid PRIMARY KEY,
    alias         character varying(255) NOT NULL,
    first_name    character varying(255),
    last_name     character varying(255),
    phone_number  character varying(255) NOT NULL,
    language_code character varying(5) NOT NULL, -- ISO 639-1 standard language codes
    avatar        text,
    country_code  character varying(2), -- ISO 3166-1 alpha-2
    created_at    timestamp(0) without time zone NOT NULL,
    updated_at    timestamp(0) without time zone NOT NULL,
    accessed_at   timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX users_id_index ON users (id uuid_ops);
CREATE UNIQUE INDEX users_alias_index ON users (alias text_ops);
CREATE UNIQUE INDEX users_phone_number_index ON users (phone_number text_ops);

-- Chats

CREATE TYPE ChatType AS ENUM ('private', 'group', 'channel');

CREATE TABLE chats
(
    id          uuid PRIMARY KEY,
    type        ChatType NOT NULL,
    title       character varying(255) NOT NULL,
    description text,
    avatar      text,
    created_at  timestamp(0) without time zone NOT NULL,
    updated_at  timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX chats_id_index ON chats (id uuid_ops);

-- Chat Participants

CREATE TYPE ChatParticipantRole AS ENUM ('admin', 'reader', 'writer', 'banned');

CREATE TABLE chat_participant
(
    id          uuid PRIMARY KEY,
    chat_id     uuid REFERENCES chats(id) NOT NULL,
    participant character varying(255) NOT NULL, -- User alias
    role        ChatParticipantRole NOT NULL,
    created_at  timestamp(0) without time zone NOT NULL,
    updated_at  timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX chat_participant_id_index ON chat_participant (id uuid_ops);
CREATE UNIQUE INDEX chat_participant_chat_id_and_participant_index ON chat_participant (chat_id, participant);

-- Messages

CREATE TABLE chat_messages
(
    id         uuid PRIMARY KEY,
    chat_id    uuid REFERENCES chats(id) NOT NULL,
    sender     character varying(255) NOT NULL, -- User alias
    message    text NOT NULL,
    parent_id  uuid REFERENCES chat_messages(id),
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL,
    deleted_at timestamp(0) without time zone
);
CREATE UNIQUE INDEX chat_messages_id_index ON chat_messages (id uuid_ops);
CREATE UNIQUE INDEX chat_messages_id_and_chat_id_index ON chat_messages (id, chat_id);