use std::sync::{Arc, Mutex};
use iron;
use router;

use transport::http::handlers::{pair_setup, pair_verify, accessories, characteristics, pairings, identify};
use db::context::Context;
use config::Config;

pub fn chain(config: Arc<Config>, context: Arc<Mutex<Context>>) -> iron::Chain {
    let mut router = router::Router::new();

    router.post("/pair-setup", move |request: &mut iron::Request| pair_setup::pair_setup(request, &config, &context), "pair-setup");
    // router.post("/pair-verify", pair_verify::pair_verify, "pair-verify");
    // router.get("/accessories", accessories::accessories, "accessories");
    // router.get("/characteristics", characteristics::get_characteristics, "get-characteristics");
    // router.put("/characteristics", characteristics::update_characteristics, "update-characteristics");
    // router.post("/pairings", pairings::pairings, "pairings");
    // router.post("/identify", identify::identify, "identify");

    iron::Chain::new(router)
}
