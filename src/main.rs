extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn hello_world(_req: Request<Body>) -> Response<Body> {
  println!("{:?}", _req);
  Response::new(Body::from("Hello, world!"))
}

fn main() {
  let addr = ([127, 0, 0, 1], 3000).into();
  let new_svc = || {
    service_fn_ok(hello_world)
  };

  let server = Server::bind(&addr).serve(new_svc).map_err(|e| eprintln!("Server error: {}", e));

  hyper::rt::run(server);
}
