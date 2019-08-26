use cruncher_api::initialize_cruncher_api;

fn main() {
    initialize_cruncher_api();


    //GENERAL TODO's
    //TODO: Refactor out some of the clones. There's performance gains to be gotten from sharing references instead. Use Arc's
    //TODO: Figure out how to return 404's and other error codes
}
