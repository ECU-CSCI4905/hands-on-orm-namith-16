use diesel_app::{establish_connection, CrudOperations, models::{NewTask, Task}};

fn main() {
    let connection = &mut establish_connection();

    println!(" Running CRUD operations for Tasks...");

    // CREATE
    let new_task = Task {
        id: 0, // This will be ignored during insert
        title: "Complete Rust ORM".to_string(),
        description: Some("Working with Diesel and SQLite".to_string()),
        status_id: 1,
    };
    Task::create(connection, new_task);

    // READ
    Task::read_all(connection);

    // UPDATE
    let updated_task = Task {
        id: 1,
        title: "Updated Task Title".to_string(),
        description: Some("Updated description".to_string()),
        status_id: 1,
    };
    Task::update(connection, 1, updated_task);

    // DELETE
    Task::delete(connection, 1);
}
