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

CREATE TYPE ChatType AS ENUM ('private', 'group', 'channel');

CREATE TABLE chats
(
    id uuid PRIMARY KEY
);
CREATE UNIQUE INDEX chats_id_index ON users (id uuid_ops);