extern crate diesel;
extern crate sqlite_perf;

use diesel::prelude::*;
use sqlite_perf::*;

use models::*;
use schema::users::dsl::*;
use schema::users;

static mut REPEAT_COUNT: i32 = 5;
static mut BATCH_COUNT: i32 = 5;

fn get_repeat_count() -> i32 {
    unsafe {
        REPEAT_COUNT
    }
}

fn get_batch_count() -> i32 {
    unsafe {
        BATCH_COUNT
    }
}

fn get_all_data_count() -> i32 {
    unsafe {
        REPEAT_COUNT * BATCH_COUNT
    }
}

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
    for i in 0..get_repeat_count() {
        let start_index = i * get_batch_count() + 1;
        let new_users = create_update_users(start_index, get_batch_count(), "update1");
        all_users.push(new_users);
    }
    all_users
}

fn reset_data(connection : &SqliteConnection) {
    let _ = diesel::delete(users).execute(connection);
    let count = get_all_data_count();
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

#[allow(dead_code)]
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

fn create_all_update_users() -> Vec<Vec<UpdateUser>> {
    let mut all_users = Vec::new();
    for i in 0..get_repeat_count() {
        let start_index = i * get_batch_count() + 1;
        let mut new_users = create_update_users(start_index, get_batch_count(), "update2");
        for new_user in new_users.iter_mut() {
            if new_user.id % get_batch_count() >= get_batch_count() / 2 {
                new_user.id += get_all_data_count();
            }
        }
        all_users.push(new_users);
    }
    all_users
}

fn select_create_update_test_impl(connection: &SqliteConnection, new_users: Vec<UpdateUser>) {
    let ids = new_users.iter().map(|item| item.id).collect::<Vec<_>>();
    let exist_users = users.filter(id.eq_any(ids))
        .load::<User>(connection).expect("load error");

    let exist_ids = exist_users.iter().map(|item| item.id).collect::<Vec<_>>();

    let (exist_users, not_exist_users) =
        new_users
            .into_iter()
            .fold((vec![], vec![]), |mut all_items, item| {
                if exist_ids.contains(&item.id) {
                    all_items.0.push(item);
                } else {
                    all_items.1.push(item)
                }
                all_items
            });

    let _ = diesel::insert_into(users::table)
        .values(&not_exist_users)
        .execute(connection);

    let _ = connection.transaction::<_, diesel::result::Error, _>(||{
        for user in exist_users {
            diesel::update(users::table.filter(id.eq(user.id)))
                .set(user)
                .execute(connection)?;
        }
        Ok(())
    });
}

fn select_create_update_test(connection: &SqliteConnection) {
    reset_data(connection);

    let all_users = create_all_update_users();

    record_time_cost!("select create upadte");

    for new_users in all_users {
        select_create_update_test_impl(connection, new_users);
    }
}

fn new_replace_into_test(connection : &SqliteConnection) {

    reset_data(connection);

    let all_users = create_all_update_users();

    record_time_cost!("replace into");

    for new_users in all_users {
        let _ = diesel::replace_into(users::table)
        .values(&new_users)
        .execute(connection);
    }

}

fn main() {
    let connection = establish_connection();
    
    let test_cases = [(1, 1000), (2, 1000), (10, 1000), (100, 100)];
    for (new_bacth_count, new_repeat_count) in &test_cases {
        unsafe {
            BATCH_COUNT = *new_bacth_count;
            REPEAT_COUNT = *new_repeat_count;
        }

        println!("------------------------------------");
        println!("batch count: {}, repeat count: {}", new_bacth_count, new_repeat_count);
        replace_into_test(&connection);
        update_test(&connection);
        update_transaction_test(&connection);

        new_replace_into_test(&connection);
        select_create_update_test(&connection);
        println!("------------------------------------");
    }


    // test_info(&connection);
}