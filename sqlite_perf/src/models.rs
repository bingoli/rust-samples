use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Insertable, AsChangeset, Default, Debug)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub id: i32,
    pub name: &'a str,
    pub email: &'a str,
}
