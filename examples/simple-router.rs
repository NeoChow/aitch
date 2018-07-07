extern crate aitch;
extern crate http;

use aitch::{middlewares, Responder, ResponseBuilder};
use http::Request;

fn handler1(_req: Request<()>, mut resp: ResponseBuilder) -> impl Responder {
    resp.body("Handler 1!".to_owned())
}

fn handler2(_req: Request<()>, mut resp: ResponseBuilder) -> impl Responder {
    resp.body("Handler 2!".to_owned())
}

fn main() {
    let mut router = middlewares::SimpleRouter::new();
    router.register_handler("/handler1", handler1);
    router.register_handler("/handler2", handler2);
    let handler = middlewares::logging_handler(router);

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);
    aitch::servers::HyperServer::new(addr, handler).run();
}