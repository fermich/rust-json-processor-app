use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::vec::Vec;
use std::thread;
use std::thread::JoinHandle;
use handler::PayloadHandler;

#[derive(Debug)]
pub struct Message {
    pub id: u32,
    pub payload: String
}

pub struct MessageChannel {
    sender: Sender<Message>,
    consumer: MessageConsumer
}

pub struct MessageConsumer {
    thread: JoinHandle<()>
}
impl MessageConsumer {
    pub fn new(receiver: Receiver<Message>) -> MessageConsumer {
        let thread_handler = thread::spawn(move || {
            match PayloadHandler::new() {
                Some(handler) => {
                    loop {
                        match receiver.recv() {
                            Result::Ok(v) => {
                                handler.handle(&v.payload);
                            }
                            Result::Err(e) => {
                                println!("Terminating consumer due to {:?}", e);
                                break;
                            }
                        }
                    }
                },
                None => {
                    println!("Cannot create handler for consumer! Exiting...");
                }
            }
        });
        MessageConsumer{ thread: thread_handler }
    }
}

pub struct MessageRouter {
    channels: Vec<MessageChannel>
}
impl MessageRouter {
    pub fn new(consumers_count: u32) -> Option<MessageRouter> {
        match consumers_count {
            0 => None,
            _ => {
                let channels = (0..consumers_count).map(|_| -> MessageChannel {
                    let (tx, rx) = channel();
                    MessageChannel{ sender: tx, consumer: MessageConsumer::new(rx) }
                }).collect();
                Some(MessageRouter{ channels: channels })
            }
        }
    }

    pub fn route(&self, msg: Message) {
        let sender_num = msg.id as usize % self.channels.len();
        let _ = self.channels[sender_num as usize].sender.send(msg);
    }
}
