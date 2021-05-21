#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    println!("Nickel");
    
    let mut server = Nickel::new();
    server.get("/", middleware!("Hello World"));
    server.listen("127.0.0.1:8080").unwrap();
}
