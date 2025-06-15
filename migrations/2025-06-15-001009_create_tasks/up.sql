-- Your SQL goes here
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status_id INTEGER NOT NULL,
    FOREIGN KEY(status_id) REFERENCES task_statuses(id)
);
