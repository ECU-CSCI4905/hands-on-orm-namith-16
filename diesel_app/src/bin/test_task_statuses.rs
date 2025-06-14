use diesel_app::{establish_connection, CrudOperations, models::{TaskStatus}};

fn main() {
    let connection = &mut establish_connection();

    println!("Running CRUD operations for TaskStatuses...");

    // CREATE
    let new_status = TaskStatus {
        id: 0,
        name: "In Progress".to_string(),
    };
    TaskStatus::create(connection, new_status);

    // READ
    TaskStatus::read_all(connection);

    // UPDATE
    let updated_status = TaskStatus {
        id: 1,
        name: "Completed".to_string(),
    };
    TaskStatus::update(connection, 1, updated_status);

    // DELETE
    TaskStatus::delete(connection, 1);
}
