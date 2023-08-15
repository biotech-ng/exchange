-- Changing sender field type back to character varying(255) and removing foreign key
ALTER TABLE chat_messages
DROP COLUMN sender,
ADD COLUMN sender_id character varying(255) NOT NULL;
