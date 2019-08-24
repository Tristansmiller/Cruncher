pub mod token_count_models;

use super::super::diesel::prelude::*;
use diesel::pg::{PgConnection};
use diesel::result::Error;
use diesel::sql_types::{Bool};

use self::token_count_models::{InsertableTokenCount, QueryableTokenCount};
use super::super::logger::{LogLevel, Logger};
use super::super::schema::token_count;
use super::super::schema::token_count::dsl::*;

pub struct TokenCountRepo {
    logger: Logger,
}
impl TokenCountRepo {
    pub fn new() -> TokenCountRepo {
        TokenCountRepo {
            logger: Logger::new("repos.token_count_repo.rs"),
        }
    }

    pub fn get_one_by_pk(&self, db_conn: &PgConnection, search_pk: (String, String)) -> Option<QueryableTokenCount> {
        let result = token_count.find(search_pk).first(db_conn);
        match result {
            Ok(result_val) => Some(result_val),
            Err(_err) => {
                self.logger
                    .log("Failed to retrieve token_count", LogLevel::Warning);
                None
            }
        }
    }
    pub fn get_all(&self, db_conn: &PgConnection) -> Vec<QueryableTokenCount> {
        let results: Vec<QueryableTokenCount> = token_count
            .load::<QueryableTokenCount>(db_conn)
            .expect("Error loading token_count.");
        results
    }

    pub fn get_vec_by_ids(&self, db_conn: &PgConnection, search_vals: Vec<(String,String)>) -> Vec<QueryableTokenCount> {
        let always_false = Box::new(ticker.eq("".to_string()));
        let query: Box<dyn BoxableExpression<token_count::table, _, SqlType = Bool>> = search_vals
            .into_iter()
            .map(|(a,b)| ticker.eq(a).and(token.eq(b)))
            .fold(always_false, |query, item| {
                Box::new(query.or(item))
            });

        let results = token_count.filter(query)
                                        .load::<QueryableTokenCount>(db_conn)
                                        .expect("Error loading");
        results
    }

    pub fn save_one(&self, db_conn: &PgConnection, save_val: InsertableTokenCount) -> Result<QueryableTokenCount, Error> {
        let already_existing_val: Option<QueryableTokenCount> =
            self.get_one_by_pk(db_conn,(save_val.ticker.to_string(), save_val.token.to_string()));
        let result = match already_existing_val {
            Some(already_existing_val) => diesel::update(&already_existing_val)
                .set(&save_val)
                .get_result(db_conn),
            None => diesel::insert_into(token_count::table)
                .values(save_val)
                .get_result(db_conn),
        };
        result
    }
}
