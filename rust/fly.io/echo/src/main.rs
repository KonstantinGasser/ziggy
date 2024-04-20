use std::io::{BufRead, Write};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message<T> {
    src: String,
    dest: String,
    body: T,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
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

    fn handle(&mut self, msg: Message<Request>) -> Result<Message<Response>> {
        self.id_counter += 1;
        match msg.body {
            Request::Init {
                msg_id, node_id, ..
            } => {
                self.label = node_id;

                Ok(Message {
                    src: self.label.clone(),
                    dest: msg.src,
                    body: Response::InitOk {
                        in_reply_to: msg_id,
                    },
                })
            }
            Request::Echo { msg_id, echo } => Ok(Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Response::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }),
            Request::EchoOk { msg_id, echo, .. } => Ok(Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Response::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }),

            Request::InitOk { .. } => {
                bail!("unexpected message. Node should not reveive init_ok message")
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut node = Node::new();

    let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Request>>();

    for message in messages {
        let message = message.context("deserialize of input message failed")?;
        let response = node
            .handle(message)
            .context("applying message to state machine")?;

        serde_json::to_writer(&mut stdout, &response)
            .context("derserializing response to stderr")?;

        stdout.write(b"\n").context("writing newline to stdout")?;

        stdout.flush().context("flusing stderr failed")?;
    }

    Ok(())
}
