-- Your SQL goes here
CREATE TABLE IF NOT EXISTS login_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id VARCHAR NOT NULL references companies(company_id),
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL, -- old encrypted password
    history_id INT NOT NULL
)