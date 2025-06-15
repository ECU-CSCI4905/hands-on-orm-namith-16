-- Your SQL goes here
CREATE TABLE user_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    status_id INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(task_id) REFERENCES tasks(id),
    FOREIGN KEY(status_id) REFERENCES task_statuses(id)
);
