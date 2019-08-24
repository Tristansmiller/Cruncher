use std::collections::HashMap;
use diesel::PgConnection;
use diesel::prelude::*;
use std::fs;
use serde::Deserialize;
use cruncher_api::repos::stock_repo::stock_models::{InsertableStock, QueryableStock};
use cruncher_api::repos::token_count_repo::token_count_models::{InsertableTokenCount, QueryableTokenCount};
use crate::schema;
use std::error::Error;
use diesel::result::DatabaseErrorKind;

#[derive(Deserialize)]
struct TokenCountedStockInfo {
    stocks: Vec<TokenCountedStock>,
}

#[derive(Deserialize, Clone, Debug)]
struct TokenCountedStock {
    ticker: String,
    name: String,
    legal_name: String,
    sic: String,
    stock_exchange: String,
    token_count: HashMap<String, i32>,
}
pub fn create_stock<'a>(
    conn: &PgConnection,
    insertable_stock: InsertableStock,
) {
    use schema::stocks;
    use schema::stocks::dsl::*;
    use diesel::result::Error;
    let res: Result<QueryableStock, Error> = diesel::insert_into(stocks::table)
        .values(&insertable_stock)
        .get_result(conn);
    match res {
        Ok(_) => (),
        Err(_) => ()
    }
}
pub fn create_token_count<'a>(
    conn: &PgConnection,
    insertable_token_count: InsertableTokenCount,
) {
    use schema::token_count;
    use schema::token_count::dsl::*;
    use diesel::result::Error;
    let res : Result<QueryableTokenCount,Error> = diesel::insert_into(token_count::table)
        .values(&insertable_token_count)
        .get_result(conn);
    match res {
        Ok(_)=>(),
        Err(_)=>()
    }
}

pub fn refreshDB(db_conn: &PgConnection) {
    let raw_json_string: String =
        fs::read_to_string("./TokenCountedStocksUnfiltered.json").expect("Something went wrong reading the file");
    let token_counted_stocks: TokenCountedStockInfo =
        serde_json::from_str(&raw_json_string).expect("Could not parse file");

    let insertable_stocks:Vec<InsertableStock>  = token_counted_stocks.stocks
                                                                        .iter()
                                                                        .map(|token_counted_stock| {
                                                                            InsertableStock {
                                                                                ticker: token_counted_stock.ticker.as_str(),
                                                                                name: token_counted_stock.name.as_str(),
                                                                                legal_name: token_counted_stock.legal_name.as_str(),
                                                                                sic: token_counted_stock.sic.as_str(),
                                                                                stock_exchange: token_counted_stock.stock_exchange.as_str()
                                                                            }
                                                                        }).collect();
    let insertable_token_counts: Vec<InsertableTokenCount>
                                = token_counted_stocks.stocks
                                     .iter()
                                     .map(|token_counted_stock| {
                                         let ticker = &token_counted_stock.ticker;
                                         let insertable_token_counts: Vec<InsertableTokenCount> = token_counted_stock.token_count
                                                                             .iter()
                                                                             .map(|(token,count)|{
                                             InsertableTokenCount {
                                                 ticker: ticker.as_str(),
                                                 token: token.as_str(),
                                                 count: count,
                                             }
                                         }).collect();
                                         insertable_token_counts
                                     }).flatten().collect();
    for insertable_stock in insertable_stocks {
        create_stock(db_conn, insertable_stock);
    }
    for insertable_token_count in insertable_token_counts {
        create_token_count(db_conn, insertable_token_count);
    }

}