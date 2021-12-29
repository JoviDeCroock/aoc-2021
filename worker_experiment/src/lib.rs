use worker::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
mod utils;

#[derive(Deserialize, Serialize)]
struct Query {
  query: String
}

fn calculate_hash(t: &String) -> u64 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |_req, _ctx| async move {
          Response::ok("We currently only support POST")
        })
        .post_async("/", |mut req, _ctx| async move {
          let data: Query;
          match req.json().await {
            Ok(res) => data = res,
            Err(_) => return Response::error("Bad request", 400),
          }

          let query: String = data.query;
          let hash: u64 = calculate_hash(&query);
          console_log!(
            "{} {} from region: {} and called with query: {} with hash: {}",
            req.method().to_string(),
            req.path(),
            req.cf().region().unwrap_or("unknown region".into()),
            query,
            hash
          );

          // Differentiate between query and mutation
          // Query: Try and get hash from KV
          // --> Found return value with 200
          // --> Unfound fetch from endpoint, save it and return response

          // Mutation:
          // --> send mutation to endpoint, analyze it and evict entities

          Response::ok("Hello world")
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
