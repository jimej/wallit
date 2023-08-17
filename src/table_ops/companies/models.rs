use crate::schema::companies;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Company {
    pub id: i32,
    pub company_id: String,
    pub url: String,
    pub testcol: String,
}

#[derive(Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany<'a> {
    pub company_id: &'a str,
    pub url: &'a str,
    pub testcol: &'a str,
}
