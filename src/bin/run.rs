extern crate docopt;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate simple_rest_api;

use docopt::Docopt;
use futures::{future, Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use simple_rest_api::*;

const USAGE: &'static str = "
Usage: run <bind_address>
";

fn respond(req: Request<Body>) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    match (req.method(), req.uri().path()) {
        // Only accept POST at route /
        (&Method::POST, "/") => {
            Box::new(req.into_body().concat2().map(|b| {
                // Parse the request body as JSON
                let readings: Vec<models::NewReading> = match serde_json::from_slice(&b) {
                    Ok(readings) => readings,
                    Err(_error) => {
                        let body = format!("Couldn't process JSON");
                        return Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(body.into())
                            .unwrap();
                    }
                };

                let connection = establish_connection();
                readings.into_iter().for_each(|reading| {
                    let r = create_reading(&connection, reading);
                    println!("{:?}", r);
                });
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::empty())
                    .unwrap()
            }))
        }
        // 404 NotFound
        _ => {
            println!("Got a {} hit at {}", req.method(), req.uri().path());
            let body = format!("Route {} was not found on this server", req.uri().path());
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body.into())
                    .unwrap(),
            ))
        }
    }
}

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());
    let mut ip_address = [0u8; 4];
    for (i, ip_bit) in args.get_str("<bind_address>").split('.').enumerate() {
        match ip_bit.parse::<u8>() {
            Ok(ip) => ip_address[i] = ip,
            Err(_) => panic!("Couldn't parse specified IP bind address"),
        };
    }
    let addr = (ip_address, 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(respond))
        .map_err(|e| eprintln!("Server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
