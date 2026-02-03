-- Supprimer les index
DROP INDEX IF EXISTS idx_messages_created_at;
DROP INDEX IF EXISTS idx_messages_author_id;
DROP INDEX IF EXISTS idx_messages_channel_id;
DROP INDEX IF EXISTS idx_channels_server_id;
DROP INDEX IF EXISTS idx_server_members_server_id;
DROP INDEX IF EXISTS idx_server_members_user_id;

-- Supprimer les tables dans l'ordre inverse (à cause des foreign keys)
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS channels;
DROP TABLE IF EXISTS server_members;
DROP TABLE IF EXISTS servers;
DROP TABLE IF EXISTS users;

-- Supprimer le type ENUM
DROP TYPE IF EXISTS user_role;
