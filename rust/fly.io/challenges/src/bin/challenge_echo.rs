use anyhow::Result;
use serde::{Deserialize, Serialize};

use challenges::{event_loop, Handle, Message};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Echo { msg_id: usize, echo: String },
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
}

struct EchoHandler {
    label: String,
    id_counter: usize,
}

impl Handle<Request, Response> for EchoHandler {
    fn new(label: &str) -> Self {
        EchoHandler {
            label: label.to_string(),
            id_counter: 0,
        }
    }

    fn handle(&mut self, msg: Message<Request>) -> Result<Message<Response>> {
        self.id_counter += 1;
        match msg.body {
            Request::Echo { msg_id, echo } => Ok(Message {
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
    event_loop::<EchoHandler, Request, Response>()
}
