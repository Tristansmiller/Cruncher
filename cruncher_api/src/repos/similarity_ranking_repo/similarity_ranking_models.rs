use super::super::super::schema::similarity_ranking;
#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(tickera, tickerb)]
#[table_name = "similarity_ranking"]
pub struct QueryableSimilarityRanking {
    pub tickera: String,
    pub tickerb: String,
    pub similarity: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "similarity_ranking"]
pub struct InsertableSimilarityRanking<'a> {
    pub tickera: &'a str,
    pub tickerb: &'a str,
    pub similarity: &'a i32,
}