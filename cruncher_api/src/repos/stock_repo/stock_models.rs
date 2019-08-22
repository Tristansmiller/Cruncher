use super::super::super::schema::stocks;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(ticker)]
#[table_name = "stocks"]
pub struct QueryableStock {
    pub ticker: String,
    pub name: Option<String>,
    pub legal_name: Option<String>,
    pub sic: Option<String>,
    pub stock_exchange: Option<String>
}


#[derive(Insertable, AsChangeset)]
#[table_name = "stocks"]
pub struct InsertableStock<'a> {
    pub ticker: &'a str,
    pub name: &'a str,
    pub legal_name: &'a str,
    pub sic: &'a str,
    pub stock_exchange: &'a str,
}



