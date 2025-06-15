use diesel_app::{establish_connection, CrudOperations, models::TaskStatus};

fn main() {
    let connection = &mut establish_connection();
    println!("Running CRUD operations for TaskStatuses...");

    // CREATE
    let new_status = TaskStatus {
        id: 0,
        name: "Pending".to_string(),
    };
    let _ = TaskStatus::create(connection, new_status);

    // READ
    let _ = TaskStatus::read_all(connection);

    // UPDATE
    let updated_status = TaskStatus {
        id: 1,
        name: "In Progress".to_string(),
    };
    let _ = TaskStatus::update(connection, 1, updated_status);

    // DELETE
    let _ = TaskStatus::delete(connection, 1);
}
