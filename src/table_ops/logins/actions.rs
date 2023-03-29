use super::models::*;
// use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

pub fn add_login(conn: &mut SqliteConnection, login: &NewLogin) -> usize {
    use crate::schema::logins;
    // let new_company = NewCompany { company_id, url };

    diesel::insert_into(logins::table)
        .values(login)
        .execute(conn) // get_results doesn't work with sqlite
        .expect("Failed to create new post")
}

use crate::schema::logins::dsl::*;
pub fn reveal(conn: &mut SqliteConnection, company: &str) -> Vec<Login> {
    logins
        .filter(company_id.eq(company))
        .limit(1)
        .load::<Login>(conn)
        .expect("Error loading login")
}
