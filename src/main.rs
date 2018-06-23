extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn respond(req: Request<Body>) -> BoxFut {
  let mut response = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    // Only accept POST at route /
    (&Method::POST, "/") => {
      let mapping = req.into_body().map(|chunk| {
        chunk
          .iter()
          .map(|byte| byte.to_ascii_uppercase())
          .collect::<Vec<u8>>()
      });
      *response.body_mut() = Body::wrap_stream(mapping);
    }
    // 404 NotFound
    _ => {
      println!("Got a {} hit at {}", req.method(), req.uri().path());
      *response.body_mut() = Body::from(format!("Route {} was not found on this server", req.uri().path()));
      *response.status_mut() = StatusCode::NOT_FOUND;
    }
  }

  Box::new(future::ok(response))
}

fn main() {
  let addr = ([127, 0, 0, 1], 3000).into();

  let server = Server::bind(&addr)
    .serve(|| { service_fn(respond) })
    .map_err(|e| eprintln!("Server error: {}", e));

  println!("Listening on http://{}", addr);
  hyper::rt::run(server);
}
