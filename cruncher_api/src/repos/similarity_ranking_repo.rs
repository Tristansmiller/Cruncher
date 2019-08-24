pub mod similarity_ranking_models;
use self::similarity_ranking_models::{InsertableSimilarityRanking, QueryableSimilarityRanking};
use super::super::diesel::prelude::*;
use super::super::logger::{LogLevel, Logger};
use super::super::schema::similarity_ranking;
use super::super::schema::similarity_ranking::dsl::*;
use diesel::pg::{PgConnection};
use diesel::result::Error;
use diesel::sql_types::{Bool};

pub struct SimilarityRankingRepo {
    logger: Logger,
}
impl SimilarityRankingRepo {
    pub fn new() -> SimilarityRankingRepo {
        SimilarityRankingRepo {
            logger: Logger::new("repos.stock_repos.rs"),
        }
    }

    pub fn get_one_by_pk(&self, db_conn: &PgConnection, search_pk: (String, String)) -> Option<QueryableSimilarityRanking> {
        let result = similarity_ranking.find(search_pk).first(db_conn);
        match result {
            Ok(result_val) => Some(result_val),
            Err(err) => {
                self.logger
                    .log("Failed to retrieve similarity_ranking", LogLevel::Warning);
                None
            }
        }
    }
    pub fn get_all(&self, db_conn: &PgConnection,) -> Vec<QueryableSimilarityRanking> {
        let results: Vec<QueryableSimilarityRanking> = similarity_ranking
            .load::<QueryableSimilarityRanking>(db_conn)
            .expect("Error loading similarity_ranking.");
        results
    }

    pub fn get_vec_by_ids(&self, db_conn: &PgConnection, search_vals: Vec<(String,String)>) -> Vec<QueryableSimilarityRanking> {
        let always_false = Box::new(tickera.eq("".to_string()));
        let query: Box<dyn BoxableExpression<similarity_ranking::table, _, SqlType = Bool>> = search_vals
            .into_iter()
            .map(|(a,b)| tickera.eq(a).and(tickerb.eq(b)))
            .fold(always_false, |query, item| {
                Box::new(query.or(item))
            });

        let results = similarity_ranking.filter(query)
                                        .load::<QueryableSimilarityRanking>(db_conn)
                                        .expect("Error loading");
        results
    }

    pub fn save_one(&self, db_conn: &PgConnection, save_val: InsertableSimilarityRanking) -> Result<QueryableSimilarityRanking, Error> {
        let already_existing_val: Option<QueryableSimilarityRanking> =
            self.get_one_by_pk(db_conn,(save_val.tickera.to_string(),save_val.tickerb.to_string()));
        let result = match already_existing_val {
            Some(already_existing_val) => diesel::update(&already_existing_val)
                .set(&save_val)
                .get_result(db_conn),
            None => diesel::insert_into(similarity_ranking::table)
                .values(save_val)
                .get_result(db_conn),
        };
        result
    }
}
