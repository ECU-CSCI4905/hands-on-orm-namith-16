use diesel_app::{establish_connection, CrudOperations, models::UserTask};

fn main() {
    let connection = &mut establish_connection();
    println!("Running CRUD operations for UserTasks...");

    // CREATE
    let new_user_task = UserTask {
        id: 0,
        user_id: 1,
        task_id: 2,
        status_id: 1,
    };
    let _ = UserTask::create(connection, new_user_task);

    // READ
    let _ = UserTask::read_all(connection);

    // UPDATE
    let updated_user_task = UserTask {
        id: 1,
        user_id: 1,
        task_id: 2,
        status_id: 1,
    };
    let _ = UserTask::update(connection, 1, updated_user_task);

    // DELETE
    let _ = UserTask::delete(connection, 1);
    
}
