use crate::CruncherDbConn;
use crate::CruncherManager;
use rocket::State;
use crate::db_refresh;
#[get("/get/<id>")]
pub fn get_stock_by_id(conn: CruncherDbConn, cruncher_manager: State<CruncherManager>, id: String) -> String {
    let results = cruncher_manager.stock_similarity_service.getStockById(&*conn,id);
    serde_json::to_string(&results).expect("Failed to retrieve")
}
#[get("/getall")]
pub fn get_all(conn: CruncherDbConn, cruncher_manager: State<CruncherManager>) -> String {
  //  let results = cruncher_manager.stock_similarity_service.getAllStocks(&*conn);
    let results = cruncher_manager.stock_similarity_service.getAllTokenCounts(&*conn);
    serde_json::to_string(&results).expect("Failed to retrieve")
}