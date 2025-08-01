// @generated automatically by Diesel CLI.

diesel::table! {
    task_statuses (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status_id -> Integer,
    }
}

diesel::table! {
    user_tasks (id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
        status_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        active -> Bool,
    }
}

diesel::joinable!(tasks -> task_statuses (status_id));
diesel::joinable!(user_tasks -> task_statuses (status_id));
diesel::joinable!(user_tasks -> tasks (task_id));
diesel::joinable!(user_tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    task_statuses,
    tasks,
    user_tasks,
    users,
);
