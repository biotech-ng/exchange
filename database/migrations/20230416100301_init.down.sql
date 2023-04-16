-- Chat Participants

DROP INDEX chat_participant_chat_id_and_participant_index;
DROP INDEX chat_participant_id_index;
DROP TABLE chat_participant;

DROP TYPE ChatParticipantRole;

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