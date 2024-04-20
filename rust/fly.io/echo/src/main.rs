use std::io::{BufRead, Write};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Payload,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Payload {
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

    fn handle(&mut self, msg: Message) -> Result<Message> {
        self.id_counter += 1;
        match msg.body {
            Payload::Init {
                msg_id, node_id, ..
            } => {
                self.label = node_id;

                Ok(Message {
                    src: self.label.clone(),
                    dest: msg.src,
                    body: Payload::InitOk {
                        in_reply_to: msg_id,
                    },
                })
            }
            Payload::Echo { msg_id, echo } => Ok(Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Payload::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }),
            Payload::EchoOk { msg_id, echo, .. } => Ok(Message {
                src: self.label.clone(),
                dest: msg.src,
                body: Payload::EchoOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    echo,
                },
            }),

            Payload::InitOk { .. } => {
                bail!("unexpected message. Node should not reveive init_ok message")
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut node = Node::new();

    let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

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
