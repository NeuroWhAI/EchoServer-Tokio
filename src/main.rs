extern crate tokio_proto;

mod protocol;
mod service;

use tokio_proto::TcpServer;

use protocol::proto;
use service::echo;

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto::Line, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(echo::EchoRev));
}

