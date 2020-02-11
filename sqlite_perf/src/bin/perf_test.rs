extern crate diesel;
extern crate sqlite_perf;

use diesel::prelude::*;
use sqlite_perf::*;

use models::*;
use schema::users::dsl::*;
use schema::users;

const REPEAT_TIME: i32 = 100;
const BATCH_COUNT: i32 = 100;

fn create_update_users(start_index: i32, count: i32, suffix: &str) -> Vec<UpdateUser> {
    let mut new_users = Vec::new();
    for i in start_index..(start_index + count) {
        let i_string = i.to_string();
        let new_name: String = "name".to_string() + &i_string + suffix;
        let new_email: String = new_name.clone() + "@github.com";
        let user = UpdateUser {id:i, name:new_name, email:new_email};
        new_users.push(user);
    }
    return new_users;
}

fn create_all_users() -> Vec<Vec<UpdateUser>> {
    let mut all_users = Vec::new();
    for i in 0..REPEAT_TIME {
        let start_index = i * BATCH_COUNT + 1;
        let new_users = create_update_users(start_index, BATCH_COUNT, "update1");
        all_users.push(new_users);
    }
    all_users
}

fn reset_data(connection : &SqliteConnection) {
    let _ = diesel::delete(users).execute(connection);
    let count = REPEAT_TIME * BATCH_COUNT;
    let new_users = create_update_users(1, count, "");
    let _ = diesel::replace_into(users::table)
        .values(&new_users)
        .execute(connection);
}

fn replace_into_test(connection : &SqliteConnection) {

    reset_data(connection);

    let all_users = create_all_users();

    record_time_cost!("replace into");

    for new_users in all_users {
        let _ = diesel::replace_into(users::table)
            .values(&new_users)
            .execute(connection);
    }
}

fn update_test(connection: &SqliteConnection) {
    reset_data(connection);

    let all_users = create_all_users();

    record_time_cost!("update");

    for new_users in all_users {
        for user in new_users {
            let _ = diesel::update(users::table.filter(id.eq(user.id)))
                .set(user)
                .execute(connection);
        }
    }
}

fn update_transaction_test(connection: &SqliteConnection) {
    reset_data(connection);

    let all_users = create_all_users();

    record_time_cost!("update by transaction");

    for new_users in all_users {
        let _ = connection.transaction::<_, diesel::result::Error, _>(||{
            for user in new_users {
                diesel::update(users::table.filter(id.eq(user.id)))
                    .set(user)
                    .execute(connection)?;
            }
            Ok(())
        });
    }
}

fn test_info(connection : &SqliteConnection) {

    let count = users.count()
        .get_result::<i64>(connection)
        .expect("Error loading users");

    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("----------------------------");
    println!("All users count: {}", count);
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} - {} - {}", user.id, user.name, user.email);
    }
}

fn main() {
    let connection = establish_connection();
    
    replace_into_test(&connection);
    update_test(&connection);
    update_transaction_test(&connection);

    test_info(&connection);
}