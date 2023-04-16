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

-- Chat Member

CREATE TYPE ChatMemberRole AS ENUM ('creator', 'admin', 'member', 'left', 'banned');

CREATE TABLE chat_member
(
    id                   uuid PRIMARY KEY,
    chat_id              uuid REFERENCES chats(id) NOT NULL,
    member               character varying(255) NOT NULL, -- User alias
    role                 ChatMemberRole NOT NULL,
    last_read_message_id uuid REFERENCES chat_messages(id),
    created_at           timestamp(0) without time zone NOT NULL,
    updated_at           timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX chat_member_id_index ON chat_member (id uuid_ops);
CREATE UNIQUE INDEX chat_member_chat_id_and_participant_index ON chat_member (chat_id, member);

-- Address

CREATE TABLE addresses
(
    id         uuid PRIMARY KEY,
    zip_code   integer NOT NULL,
    country    character varying(255) NOT NULL,
    region     character varying(255) NOT NULL,
    city       character varying(255) NOT NULL,
    district   character varying(255),
    street     character varying(255) NOT NULL,
    building   character varying(255) NOT NULL,
    apartment  character varying(255) NOT NULL,
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX addresses_id_index ON addresses (id uuid_ops);
