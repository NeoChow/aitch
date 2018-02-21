extern crate aitch;
extern crate http;

use aitch::{Request, Response, ResponseBuilder};

fn handler(_req: &mut Request, mut resp: ResponseBuilder) -> http::Result<Response> {
    resp.body("Hello world!".as_bytes().to_owned())
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    aitch::Server::new(addr, &handler).run().unwrap();
}
