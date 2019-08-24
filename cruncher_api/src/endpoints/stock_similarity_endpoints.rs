use crate::CruncherDbConn;
use rocket::State;
use rocket::get;
use crate::StockSimilarityService;

#[get("/get/<id>")]
pub fn get_stock_by_id(conn: CruncherDbConn, stock_similarity_service: State<StockSimilarityService>, id: String) -> String {
    let results = stock_similarity_service.get_stock_by_ticker(&*conn,id);
    serde_json::to_string(&results).expect("Failed to retrieve")
}

#[get("/getall")]
pub fn get_all(conn: CruncherDbConn, stock_similarity_service: State<StockSimilarityService>) -> String {
  //  let results = cruncher_manager.stock_similarity_service.getAllStocks(&*conn);
    let results = stock_similarity_service.get_all_token_counts(&*conn);
    serde_json::to_string(&results).expect("Failed to retrieve")
}

#[get("/get-similar/<ticker>")]
pub fn get_similar(conn: CruncherDbConn, stock_similarity_service: State<StockSimilarityService>, ticker: String) -> String {
    match stock_similarity_service.get_similar_stocks(&*conn, ticker) {
        Ok(ranked_stocks) => {
           serde_json::to_string(&ranked_stocks).expect("Failed to calculate rankings")
        },
        Err(_) => {
            "".to_string()
        }
    }
}