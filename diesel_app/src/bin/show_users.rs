use diesel::prelude::*;
use diesel_app::*;
use diesel_app::models::*;
use diesel_app::schema::users::dsl::*; // for filtering

fn main() {
    println!("Requesting connection...");
    let mut connection = establish_connection();
    println!("Received connection...");

    println!("Querying users...");
    let results = users
        // .filter(active.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());

    for user in results {
        println!("-----------");
        println!("Name: {}", user.name);
        println!("Email: {}", user.email);
        println!("Active: {}", user.active);
    }
    println!("-----------");
}
