use crate::schema::logins;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Login {
    pub id: i32, // u32 has issues for reveal()
    pub company_id: String,
    pub username: String,
    pub password: String, 
    pub email: String,
    pub history_id: i32, // u32 has issues for reveal()
    pub url: String,
    pub description: String,
    pub lastModified: String,
    
}

#[derive(Insertable)]
#[diesel(table_name = logins)]
pub struct NewLogin<'a> { // the field orders seem to be important here
    pub company_id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub history_id: i32, // u32 doesnt work, i8 doesn't work either
    pub url: &'a str,
    pub description: &'a str,
    pub lastModified: &'a str,
    
}
