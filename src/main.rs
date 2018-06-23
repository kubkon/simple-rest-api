#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;

use futures::{future, Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

#[derive(Debug,Serialize,Deserialize)]
pub struct Reading {
  pub timestamp: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

fn respond(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
  match (req.method(), req.uri().path()) {
    // Only accept POST at route /
    (&Method::POST, "/") => {
      Box::new(req.into_body().concat2().map(|b| {
        // Parse the request body as JSON
        let readings: Vec<Reading> = serde_json::from_slice(&b).unwrap();
        readings.iter().for_each(|reading| {
          println!("{:?}", reading);
        });
        Response::builder()
          .status(StatusCode::OK)
          .body(Body::empty())
          .unwrap()
      }))
    },
    // 404 NotFound
    _ => {
      println!("Got a {} hit at {}", req.method(), req.uri().path());
      let body = format!("Route {} was not found on this server", req.uri().path());
      Box::new(future::ok(Response::builder()
                          .status(StatusCode::NOT_FOUND)
                          .body(body.into())
                          .unwrap()))
    }
  }
}

fn main() {
  let addr = ([127, 0, 0, 1], 3000).into();

  let server = Server::bind(&addr)
    .serve(|| { service_fn(respond) })
    .map_err(|e| eprintln!("Server error: {}", e));

  println!("Listening on http://{}", addr);
  hyper::rt::run(server);
}
