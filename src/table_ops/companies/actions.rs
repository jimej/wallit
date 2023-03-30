use super::models::*;
// use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

pub fn add_company(conn: &mut SqliteConnection, companyid: &str, url: &str) -> usize {
    use crate::schema::companies;
    // use models::*;

    let new_company = NewCompany {
        company_id: companyid,
        url,
    };
    diesel::insert_into(companies::table)
        .values(&new_company)
        .execute(conn) // get_results doesn't work with sqlite
        .expect("Failed to add the new company")
}
