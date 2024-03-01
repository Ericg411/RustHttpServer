#![allow(dead_code, unused_variables, unused_imports)]
mod server;
mod http;

use server::Server;
use http::request;
use http::method;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
