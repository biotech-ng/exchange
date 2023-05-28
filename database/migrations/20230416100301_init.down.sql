-- Project Member

DROP INDEX project_members_company_id_and_user_id_index;
DROP INDEX project_members_id_index;

DROP TABLE project_members;

-- Company projects

DROP INDEX company_projects_company_id_and_project_id_index;
DROP INDEX company_projects_id_index;

DROP TABLE company_projects;

-- Projects

DROP INDEX projects_id_index;

DROP TABLE projects;

-- Company Member

DROP INDEX company_members_user_id_and_company_id_index;
DROP INDEX company_members_id_index;

DROP TABLE company_members;

-- Company

DROP INDEX companies_name_and_address_id_index;
DROP INDEX companies_id_index;

DROP TABLE companies;

-- Chats

DROP INDEX addresses_id_index;
DROP TABLE addresses;

-- Chat Participants

DROP INDEX chat_member_chat_id_and_participant_index;
DROP INDEX chat_member_id_index;
DROP TABLE chat_member;

DROP TYPE ChatMemberRole;

-- Messages

DROP INDEX chat_messages_id_and_chat_id_index;
DROP INDEX chat_messages_id_index;
DROP TABLE chat_messages;

-- Chats

DROP INDEX chats_id_index;
DROP TABLE chats;

DROP TYPE ChatType;

-- Users

DROP INDEX users_phone_number_index;
DROP INDEX users_alias_index;
DROP INDEX users_id_index;
DROP TABLE users;