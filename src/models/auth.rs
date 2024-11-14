use crate::schema::schema::auth;
use diesel::Insertable;

pub struct Auth {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
