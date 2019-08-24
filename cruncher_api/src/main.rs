use cruncher_api::initialize_cruncher_api;

fn main() {
    initialize_cruncher_api();
    //TODO: Figure out how to return 404's and other error codes
    //TODO: Build out SQL DB to store TokenCountedStock
    //TODO: Build out Diesel to retrieve TokenCountedStock
    //TODO: Add Rocket endpoint that does the processing and returns ranking JSON
}
