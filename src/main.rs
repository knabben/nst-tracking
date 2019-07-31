mod endpoint;
mod blockchain;
mod payload;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate time;
extern crate sawtooth_sdk;
extern crate crypto;
extern crate futures;

use crate::futures::Stream;

use futures::{future, Future};
use hyper::{Server};
use hyper::{Body, Method, Error, StatusCode, Request, Response};
use hyper::service::service_fn;


pub fn microservice_handler(req: Request<Body>)
    -> Box<Future<Item=Response<Body>, Error=Error> + Send>
{
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api/agent") => {
            let body = req.into_body().concat2()
                .map(|chunks| {
                    println!("{:?}", chunks);
                    let res = serde_json::from_slice::<endpoint::AgentRequest>(chunks.as_ref());
                    match res {
                        Ok(body) => {
                            println!("{:?}", body);
                            blockchain::run(body.username, body.password);
                            Response::new(Body::empty())
                        },
                        Err(err) => {
                            Response::builder()
                                .status(StatusCode::UNPROCESSABLE_ENTITY)
                                .body(err.to_string().into())
                                .unwrap()
                        },
                    }
                });//.and_then(|resp| serde_json::to_string(&resp));;
            Box::new(future::ok(body))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            Box::new(future::ok(response))
        }
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 8081).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(||
        service_fn(microservice_handler)
    );
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
