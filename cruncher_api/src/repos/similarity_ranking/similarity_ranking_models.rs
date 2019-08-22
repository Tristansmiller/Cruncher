use super::super::super::schema::similarity_ranking;
#[derive(Queryable)]
pub struct QueryableSimilarityRanking {
    pub tickera: String,
    pub tickerb: String,
    pub similarity: Numeric,
}

#[derive(Insertable)]
#[table_name = "similarity_ranking"]
pub struct InsertableSimilarityRanking<'a> {
    pub tickera: &'a str,
    pub tickerb: &'a str,
    pub similarity: &'a f32,
}