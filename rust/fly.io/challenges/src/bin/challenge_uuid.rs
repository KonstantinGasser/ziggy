use challenges::{stream, Handle, Message};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Generate { msg_id: usize },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Response {
    GenerateOk {
        in_reply_to: usize,
        msg_id: usize,
        id: String,
    },
}

struct GeneratorNode {
    label: String,
    id_counter: usize,
}

impl Handle<Request, Response> for GeneratorNode {
    fn new(label: &str) -> Self {
        GeneratorNode {
            label: label.to_string(),
            id_counter: 0,
        }
    }

    fn handle(&mut self, message: Message<Request>) -> anyhow::Result<Message<Response>> {
        self.id_counter += 1;
        match message.body {
            Request::Generate { msg_id, .. } => Ok(Message {
                src: self.label.clone(),
                dest: message.src,
                body: Response::GenerateOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    id: uuid::Uuid::new_v4().to_string(),
                },
            }),
        }
    }
}

fn main() -> anyhow::Result<()> {
    stream::<GeneratorNode, Request, Response>()
}
