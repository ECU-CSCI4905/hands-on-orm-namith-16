use diesel_app::{
    establish_connection,
    CrudOperations,
    models::Task,
};

fn main() {
    let connection = &mut establish_connection();

    println!("Running CRUD operations for Tasks...");

    // CREATE
    let new_task = Task {
        id: 0, // This will be ignored during insert
        title: "Complete Rust ORM".to_string(),
        description: Some("Working with Diesel and SQLite".to_string()),
        status_id: 1,
    };

    match Task::create(connection, new_task) {
        Ok(_) => println!("Task created successfully."),
        Err(e) => eprintln!("Error creating task: {}", e),
    }

    // READ
    match Task::read_all(connection) {
        Ok(tasks) => {
            println!("Tasks retrieved successfully:");
            for task in tasks {
                println!("{:?}", task);
            }
        }
        Err(e) => eprintln!("Error reading tasks: {}", e),
    }

    // UPDATE
    let updated_task = Task {
        id: 1,
        title: "Updated Task Title".to_string(),
        description: Some("Updated description".to_string()),
        status_id: 1,
    };

    match Task::update(connection, 1, updated_task) {
        Ok(_) => println!("Task updated successfully."),
        Err(e) => eprintln!("Error updating task: {}", e),
    }

    // DELETE
    match Task::delete(connection, 1) {
        Ok(_) => println!("Task deleted successfully."),
        Err(e) => eprintln!("Error deleting task: {}", e),
    }
}
