-- Messages

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