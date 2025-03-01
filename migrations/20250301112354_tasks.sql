-- Add migration script here
CREATE TABLE IF NOT EXISTS Tasks(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title varchar(250) NOT NULL,
    description varchar(1000),
    completed bit default 0
)