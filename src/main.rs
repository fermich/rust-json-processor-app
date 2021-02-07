extern crate regex;
extern crate rustc_serialize;

mod handler;
mod server;
mod messaging;
mod lifetimes;

use server::JsonMessageServer;
use messaging::MessageRouter;


fn main() {    
//    lifetimes::main2();

    let num_of_consumers = 8;
    match MessageRouter::new(num_of_consumers) {
        Some(router) => {
            match JsonMessageServer::listen() {
                Some(server) => {
                    server.start_receiving(&router);
                },
                None => println!("Cannot start server! Exiting...")
            }
            drop(router);
        },
        None => println!("Cannot create message router! Exiting...")
    }
}
