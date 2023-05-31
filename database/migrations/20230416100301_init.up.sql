-- Users

CREATE TABLE users
(
    id                    uuid PRIMARY KEY,
    alias                 character varying(255),
    first_name            character varying(255),
    last_name             character varying(255),
    email                 character varying(320) NOT NULL, -- RFC 3696, "Application Techniques for Checking and Transformation of Names"
    password_salt         character varying(88) NOT NULL,
    password_sha512       character varying(88) NOT NULL,
    access_token          text NOT NULL,
    phone_number          character varying(15), -- ITU-T E. 164
    language_code         character varying(5) NOT NULL, -- ISO 639-1 standard language codes
    avatar                text,
    country_code          character varying(2), -- ISO 3166-1 alpha-2
    created_at            timestamp(0) without time zone NOT NULL,
    updated_at            timestamp(0) without time zone NOT NULL,
    accessed_at           timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX users_id_index ON users (id uuid_ops);
CREATE UNIQUE INDEX users_alias_index ON users (alias text_ops);
CREATE UNIQUE INDEX users_email_index ON users (email text_ops);
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
    country    character varying(2) NOT NULL, -- ISO 3166, Alpha-2
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

-- Company

CREATE TABLE companies
(
    id         uuid PRIMARY KEY,
    name       character varying(255) NOT NULL,
    address_id uuid REFERENCES addresses(id) NOT NULL,
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX companies_id_index ON companies (id uuid_ops);
CREATE UNIQUE INDEX companies_name_and_address_id_index ON companies (name, address_id);

-- Company Member

CREATE TABLE company_members
(
    id         uuid PRIMARY KEY,
    user_id    uuid REFERENCES users(id) NOT NULL,
    company_id uuid REFERENCES companies(id) NOT NULL,
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX company_members_id_index ON company_members (id uuid_ops);
CREATE UNIQUE INDEX company_members_user_id_and_company_id_index ON company_members (user_id, company_id);

-- Project

CREATE TABLE projects
(
    id          uuid PRIMARY KEY,
    name        character varying(255) NOT NULL,
    description text NOT NULL,
    created_at  timestamp(0) without time zone NOT NULL,
    updated_at  timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX projects_id_index ON companies (id uuid_ops);

-- Company projects

CREATE TABLE company_projects
(
    id         uuid PRIMARY KEY,
    company_id uuid REFERENCES companies(id) NOT NULL,
    project_id uuid REFERENCES projects(id) NOT NULL,
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX company_projects_id_index ON company_projects (id uuid_ops);
CREATE UNIQUE INDEX company_projects_company_id_and_project_id_index ON company_projects (company_id, project_id);

-- Project Member

CREATE TABLE project_members
(
    id         uuid PRIMARY KEY,
    project_id uuid REFERENCES projects(id) NOT NULL,
    user_id    uuid REFERENCES users(id) NOT NULL,
    created_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX project_members_id_index ON project_members (id uuid_ops);
CREATE UNIQUE INDEX project_members_company_id_and_user_id_index ON project_members (project_id, user_id);