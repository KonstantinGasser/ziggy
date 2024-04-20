use anyhow::Result;
use serde::{Deserialize, Serialize};

mod event;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Response {
    InitOk {
        in_reply_to: usize,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
}

struct Node {
    label: String,
    id_counter: usize,
}
impl Node {
    fn new() -> Self {
        Node {
            label: String::new(),
            id_counter: 0,
        }
    }
}

impl event::Handle<Request, Response> for Node {
    fn handle(&mut self, msg: event::Message<Request>) -> Result<event::Message<Response>> {
        self.id_counter += 1;
        match msg.body {
            Request::Init {
                msg_id, node_id, ..
            } => {
                self.label = node_id;

                Ok(event::Message {
                    src: self.label.clone(),
                    dest: msg.src,
                    body: Response::InitOk {
                        in_reply_to: msg_id,
                    },
                })
            }
            Request::Echo { msg_id, echo } => Ok(event::Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Response::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }),
        }
    }
}

fn main() -> Result<()> {
    let node = Node::new();

    event::stream::<Request, Response>(node)
}
