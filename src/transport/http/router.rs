use std::sync::{Arc, Mutex};
use iron;
use router;

use transport::http::handlers::{pair_setup, pair_verify, /*accessories, characteristics, pairings, identify*/};
use db::context::Context;
use db::storage::Storage;
use db::database::Database;
use config::Config;

pub fn chain<D: 'static + Storage + Send>(config: Arc<Config>, context: Arc<Mutex<Context>>, database: Arc<Mutex<Database<D>>>) -> iron::Chain {
    let mut router = router::Router::new();

    router.post("/pair-setup", {
        let config = config.clone();
        let context = context.clone();
        let database = database.clone();
        move |request: &mut iron::Request| pair_setup::pair_setup(request, &config, &context, &database)
    }, "pair-setup");

    router.post("/pair-verify", {
        let config = config.clone();
        let context = context.clone();
        let database = database.clone();
        move |request: &mut iron::Request| pair_verify::pair_verify(request, &config, &context, &database)
    }, "pair-verify");

    // router.get("/accessories", accessories::accessories, "accessories");
    // router.get("/characteristics", characteristics::get_characteristics, "get-characteristics");
    // router.put("/characteristics", characteristics::update_characteristics, "update-characteristics");
    // router.post("/pairings", pairings::pairings, "pairings");
    // router.post("/identify", identify::identify, "identify");

    iron::Chain::new(router)
}
