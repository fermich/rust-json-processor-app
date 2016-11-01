use std::net::TcpListener;
use std::io::Read;

use messaging::MessageRouter;
use messaging::Message;

pub struct JsonMessageServer {
    listener: TcpListener
}
impl JsonMessageServer {
    pub fn listen() -> Option<JsonMessageServer> {
        match TcpListener::bind("127.0.0.1:6666") {
            Result::Ok(l) => Some(JsonMessageServer { listener: l }),
            Result::Err(_) => None
        }
    }

    pub fn start_receiving(&self, router: &MessageRouter) -> () {
        let mut message_id: u32 = 0;
        for stream in self.listener.incoming() {
            message_id += 1;
            match stream {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    stream.read_to_string(&mut buffer);
                    router.route(Message{id: message_id, payload: buffer});
                }
                Err(e) => {
                    println!("Terminating server due to {}", e);
                    break;
                }
            }
        }
    }
}
