use challenges::{event_loop, Handle, Message};
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

struct GeneratorHandler {
    label: String,
    id_counter: usize,
}

impl Handle<Request, Response> for GeneratorHandler {
    fn new(label: &str) -> Self {
        GeneratorHandler {
            label: label.to_string(),
            id_counter: 0,
        }
    }

    fn handle(&mut self, message: Message<Request>) -> anyhow::Result<Vec<Message<Response>>> {
        self.id_counter += 1;
        match message.body {
            Request::Generate { msg_id, .. } => Ok(vec![Message {
                src: self.label.clone(),
                dest: message.src,
                body: Response::GenerateOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    id: format!("{}-{}", self.label.clone(), self.id_counter),
                },
            }]),
        }
    }
}

fn main() -> anyhow::Result<()> {
    event_loop::<GeneratorHandler, Request, Response>()
}
