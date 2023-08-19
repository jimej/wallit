use crate::schema::history;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct History {
    pub id: i32,
    pub company_id: String,
    pub login: String,
    pub password: String,
    pub history_id : i32,
    pub url: String,
    pub description: String,
    pub loginLastModified: String,
    pub mode: String,
    pub lastModified: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = history)]
pub struct NewHistory<'a> { // the field orders seem to be important here
    pub company_id: &'a str,
    pub login: &'a str,
    pub password: &'a str,
    pub history_id : i32,
    pub url: &'a str,
    pub description: &'a str,
    pub loginLastModified: &'a str,
    pub mode: &'a str,
    pub lastModified: &'a str,
    pub email: &'a str,
    
}
