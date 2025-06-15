use diesel::prelude::*;
use crate::models::*;
use crate::schema::{tasks, task_statuses, user_tasks};

pub trait CrudOperations {
    fn create(conn: &mut SqliteConnection, new_item: Self) -> Result<(), diesel::result::Error>
    where
        Self: Sized;
    fn read_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error>
    where
        Self: Sized;
    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) -> Result<(), diesel::result::Error>
    where
        Self: Sized;
    fn delete(conn: &mut SqliteConnection, id: i32) -> Result<(), diesel::result::Error>;
}

// ===== TASK =====
impl CrudOperations for Task {
    fn create(conn: &mut SqliteConnection, new_item: Self) -> Result<(), diesel::result::Error> {
        let new_task = NewTask {
            title: &new_item.title,
            description: new_item.description.as_deref(),
            status_id: new_item.status_id,
        };
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(conn)?;
        Ok(())
    }

    fn read_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        tasks::table.load::<Task>(conn)
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) -> Result<(), diesel::result::Error> {
        diesel::update(tasks::table.find(id))
            .set(tasks::title.eq(update_data.title))
            .execute(conn)?;
        Ok(())
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(tasks::table.find(id)).execute(conn)?;
        Ok(())
    }
}

// ===== TASKSTATUS =====
impl CrudOperations for TaskStatus {
    fn create(conn: &mut SqliteConnection, new_item: Self) -> Result<(), diesel::result::Error> {
        let new_status = NewTaskStatus {
            name: &new_item.name,
        };
        diesel::insert_into(task_statuses::table)
            .values(&new_status)
            .execute(conn)?;
        Ok(())
    }

    fn read_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        task_statuses::table.load::<TaskStatus>(conn)
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) -> Result<(), diesel::result::Error> {
        diesel::update(task_statuses::table.find(id))
            .set(task_statuses::name.eq(update_data.name))
            .execute(conn)?;
        Ok(())
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(task_statuses::table.find(id)).execute(conn)?;
        Ok(())
    }
}

// ===== USERTASK =====
impl CrudOperations for UserTask {
    fn create(conn: &mut SqliteConnection, new_item: Self) -> Result<(), diesel::result::Error> {
        let new_user_task = NewUserTask {
            user_id: new_item.user_id,
            task_id: new_item.task_id,
        };
        diesel::insert_into(user_tasks::table)
            .values(&new_user_task)
            .execute(conn)?;
        Ok(())
    }

    fn read_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        user_tasks::table.load::<UserTask>(conn)
    }

    fn update(conn: &mut SqliteConnection, id: i32, update_data: Self) -> Result<(), diesel::result::Error> {
        diesel::update(user_tasks::table.find(id))
            .set((
                user_tasks::user_id.eq(update_data.user_id),
                user_tasks::task_id.eq(update_data.task_id),
            ))
            .execute(conn)?;
        Ok(())
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(user_tasks::table.find(id)).execute(conn)?;
        Ok(())
    }
}