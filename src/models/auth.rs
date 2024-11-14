use crate::schema::schema::auths;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct Auth {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = auths)]
pub struct NewAuth<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
