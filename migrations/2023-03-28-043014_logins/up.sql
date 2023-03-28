-- Your SQL goes here
CREATE TABLE IF NOT EXISTS logins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id VARCHAR NOT NULL UNIQUE references companies(company_id),
    username VARCHAR(40) NOT NULL,
    password VARCHAR(255) NOT NULL, -- encrypted password
    email VARCHAR(255) NOT NULL,
    history_id INT NOT NULL -- to be recorded to login_history table
)