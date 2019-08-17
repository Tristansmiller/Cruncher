use super::schema::stocks;

#[derive(Queryable)]
pub struct Stock {
    pub ticker: String,
    pub stockexchange: String,
    pub tokenmap: String,
}

#[derive(Insertable)]
#[table_name = "stocks"]
pub struct NewStock<'a> {
    pub ticker: &'a str,
    pub stockexchange: &'a str,
    pub tokenmap: &'a str,
}
