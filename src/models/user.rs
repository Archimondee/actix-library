use crate::schema::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub auth_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub auth_id: i32,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
}
