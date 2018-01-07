extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use self::tokio_service::Service;
use self::futures::{future, Future};

pub struct EchoRev;

impl Service for EchoRev {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars()
            .rev()
            .collect();
        // In this case, the response is immediate.
        Box::new(future::ok(rev))
    }
}
