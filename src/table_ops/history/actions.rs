
use super::models::*;
// use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

pub fn add_history(conn: &mut SqliteConnection, l: &NewHistory) -> usize {
    use crate::schema::history;
    // let new_company = NewCompany { company_id, url };

    diesel::insert_into(history::table)
        .values(l)
        .execute(conn) // get_results doesn't work with sqlite
        .expect("Failed to create new login")
}