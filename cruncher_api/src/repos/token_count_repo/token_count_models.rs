use super::super::super::schema::token_count;
use serde::Serialize;
#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[primary_key(ticker,token)]
#[table_name = "token_count"]
pub struct QueryableTokenCount {
    pub ticker: String,
    pub token: String,
    pub count: i32,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "token_count"]
pub struct InsertableTokenCount<'a> {
    pub ticker: &'a str,
    pub token: &'a str,
    pub count: &'a i32,
}