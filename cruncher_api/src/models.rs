use super::schema::stocks;

#[derive(Queryable)]
pub struct Stock {
    pub ticker: String,
    pub name: String,
    pub legal_name: String,
    pub sic: String,
    pub stock_exchange: String
}

#[derive(Insertable)]
#[table_name = "stocks"]
pub struct InsertableStock<'a> {
    pub ticker: &'a str,
    pub name: &'a str,
    pub legal_name: &'a str,
    pub sic: &'a str,
    pub stock_exchange: &'a str
}
