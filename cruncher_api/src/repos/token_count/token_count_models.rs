use super::super::super::schema::token_count;
#[derive(Queryable)]
pub struct QueryableTokenCount {
    pub ticker: String,
    pub token: String,
    pub count: Integer,
}

#[derive(Insertable)]
#[table_name = "similarity_ranking"]
pub struct InsertableTokenCount<'a> {
    pub ticker: &'a str,
    pub token: &'a str,
    pub count: &'a u32,
}