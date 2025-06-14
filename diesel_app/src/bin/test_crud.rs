use diesel::prelude::*;
use diesel_app::*;
use diesel_app::models::*;
use diesel_app::schema::users::dsl::*;

fn main() {
    let mut conn = establish_connection();

    println!("\n CREATE NEW USER...");
    let new_user = NewUser {
        name: "David",
        email: "david@example.com",
        active: true,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error inserting user");

    println!(" Inserted David");

    println!("\n READ USERS...");
    let results = users
        .limit(10)
        .select(User::as_select())
        .load::<User>(&mut conn)
        .expect("Error loading users");

    for user in &results {
        println!(" - {} ({}) Active: {}", user.name, user.email, user.active);
    }

    println!("\n UPDATE USER...");
    let updated = diesel::update(users.filter(name.eq("David")))
        .set(active.eq(&false)) // <-- Fix is here: pass reference to bool
        .execute(&mut conn)
        .expect("Error updating user");

    println!("Updated {} record(s). David set to inactive", updated);

    println!("\n VERIFY UPDATE...");
    let updated_user = users
        .filter(name.eq("David"))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .expect("Error fetching updated user");

    assert_eq!(updated_user.active, false);
    println!(" Verified David is inactive");

    println!("\n DELETE USER...");
    diesel::delete(users.filter(name.eq("David")))
        .execute(&mut conn)
        .expect("Error deleting user");

    println!(" David deleted");

    println!("\n USERS AFTER DELETE...");
    let final_users = users
        .limit(10)
        .select(User::as_select())
        .load::<User>(&mut conn)
        .expect("Error loading users");

    for user in final_users {
        println!(" - {} ({}) Active: {}", user.name, user.email, user.active);
    }

    println!("\n  ALL CRUD OPERATIONS DONE");
}
