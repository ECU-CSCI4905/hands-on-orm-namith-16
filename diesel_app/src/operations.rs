use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub trait CrudOperations {
    fn create(conn: &mut SqliteConnection, new_item: Self) where Self: Sized;
    fn read_all(conn: &mut SqliteConnection);
    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) where Self: Sized;
    fn delete(conn: &mut SqliteConnection, id: i32);
}

// ===== TASK =====
impl CrudOperations for Task {
    fn create(conn: &mut SqliteConnection, new_item: Self) {
        let new_task = NewTask {
            title: &new_item.title,
            description: new_item.description.as_deref(),
            status_id: new_item.status_id,
        };
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(conn)
            .expect("Error inserting task");
    }

    fn read_all(conn: &mut SqliteConnection) {
        let results = tasks::table.load::<Task>(conn).expect("Error reading tasks");
        for task in results {
            println!("{:?}", task);
        }
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) {
        diesel::update(tasks::table.find(id))
            .set(tasks::title.eq(update_data.title))
            .execute(conn)
            .expect("Error updating task");
    }

    fn delete(conn: &mut SqliteConnection, id: i32) {
        diesel::delete(tasks::table.find(id))
            .execute(conn)
            .expect("Error deleting task");
    }
}

// ===== TASKSTATUS =====
impl CrudOperations for TaskStatus {
    fn create(conn: &mut SqliteConnection, new_item: Self) {
        let new_status = NewTaskStatus {
            name: &new_item.name,
        };
        diesel::insert_into(task_statuses::table)
            .values(&new_status)
            .execute(conn)
            .expect("Error inserting status");
    }

    fn read_all(conn: &mut SqliteConnection) {
        let results = task_statuses::table.load::<TaskStatus>(conn).expect("Error reading statuses");
        for status in results {
            println!("{:?}", status);
        }
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) {
        diesel::update(task_statuses::table.find(id))
            .set(task_statuses::name.eq(update_data.name))
            .execute(conn)
            .expect("Error updating status");
    }

    fn delete(conn: &mut SqliteConnection, id: i32) {
        diesel::delete(task_statuses::table.find(id))
            .execute(conn)
            .expect("Error deleting status");
    }
}

// ===== USERTASK =====
impl CrudOperations for UserTask {
    fn create(conn: &mut SqliteConnection, new_item: Self) {
        let new_user_task = NewUserTask {
            user_id: new_item.user_id,
            task_id: new_item.task_id,
        };
        diesel::insert_into(user_tasks::table)
            .values(&new_user_task)
            .execute(conn)
            .expect("Error inserting user-task");
    }

    fn read_all(conn: &mut SqliteConnection) {
        let results = user_tasks::table.load::<UserTask>(conn).expect("Error reading user-tasks");
        for ut in results {
            println!("{:?}", ut);
        }
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) {
        diesel::update(user_tasks::table.find(id))
            .set((
                user_tasks::user_id.eq(update_data.user_id),
                user_tasks::task_id.eq(update_data.task_id),
            ))
            .execute(conn)
            .expect("Error updating user-task");
    }

    fn delete(conn: &mut SqliteConnection, id: i32) {
        diesel::delete(user_tasks::table.find(id))
            .execute(conn)
            .expect("Error deleting user-task");
    }
}
