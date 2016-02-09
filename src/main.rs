extern crate daemonize;
extern crate iron;

use daemonize::{Daemonize};
use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let daemonize = Daemonize::new()
        .pid_file("tmp/iron-starter.pid");

    match daemonize.start() {
         Ok(_) => {
             println!("On 3000");
             Iron::new(hello_world).http("localhost:3000").unwrap();
         },
         Err(e) => println!("{}", e),
     }

}
