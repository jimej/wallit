ALTER TABLE logins RENAME COLUMN login TO username;
ALTER TABLE logins ADD COLUMN history_id INT;