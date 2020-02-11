extern crate diesel;
extern crate sqlite_perf;

use self::models::*;

use diesel::prelude::*;
use sqlite_perf::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    
    let results = users
    .limit(5)
    .load::<User>(&connection)
    .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} - {} - {}", user.id, user.name, user.email);
    }
}