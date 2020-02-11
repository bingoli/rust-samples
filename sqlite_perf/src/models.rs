use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset, Default, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}
