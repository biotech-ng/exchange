-- Changing the sender field type to UUID and adding a foreign key
ALTER TABLE chat_messages
DROP COLUMN sender,
ADD COLUMN sender_id uuid REFERENCES users(id) NOT NULL;