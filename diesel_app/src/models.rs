#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use crate::schema::*;

// Full User struct for reading/querying from the DB
#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

// NewUser struct for inserting into the DB
#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub active: bool,
}



#[derive(Insertable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status_id: i32,
}


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = user_tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserTask {
    pub id: i32,
    pub user_id: i32,
    pub task_id: i32,
    pub status_id: i32, // ✅ Add this field

}

#[derive(Insertable)]
#[diesel(table_name = user_tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUserTask {
    pub user_id: i32,
    pub task_id: i32,
    
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = task_statuses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TaskStatus {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = task_statuses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTaskStatus<'a> {
    pub name: &'a str,
}