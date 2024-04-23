use anyhow::Result;
use serde::{Deserialize, Serialize};

use challenges::{event_loop, Handle, Message};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Echo {
        msg_id: usize,
        echo: String,
    },
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Response {
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    InitOk {
        msg_id: usize,
        in_reply_to: usize,
    },
}

struct EchoHandler {
    label: String,
    id_counter: usize,
}

impl Handle<Request, Response> for EchoHandler {
    fn new() -> Self {
        EchoHandler {
            label: String::new(),
            id_counter: 0,
        }
    }

    fn handle(&mut self, msg: Message<Request>) -> Result<Vec<Message<Response>>> {
        self.id_counter += 1;
        match msg.body {
            Request::Init {
                msg_id, node_id, ..
            } => {
                self.label = node_id;

                Ok(vec![Message::<Response> {
                    src: self.label.clone(),
                    dest: msg.src,
                    body: Response::InitOk {
                        msg_id: self.id_counter,
                        in_reply_to: msg_id,
                    },
                }])
            }

            Request::Echo { msg_id, echo } => Ok(vec![Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Response::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }]),
        }
    }
}

fn main() -> Result<()> {
    event_loop::<EchoHandler, Request, Response>()
}
