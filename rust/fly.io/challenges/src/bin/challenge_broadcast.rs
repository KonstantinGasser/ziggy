use challenges::{event_loop, Handle, Message};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Request {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    Topology {
        msg_id: usize,
        topology: HashMap<String, Vec<String>>, // {"n1": [], "n2": [], etc.}
    },
    Broadcast {
        msg_id: usize,
        message: i64,
        idempotency_key: Option<String>,
    },
    Read {
        msg_id: usize,
    },
    // will be ignores as we do not need to react to this message
    BroadcastOk {
        msg_id: usize,
        in_reply_to: usize,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Response {
    InitOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    TopologyOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    BroadcastOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    ReadOk {
        msg_id: usize,
        in_reply_to: usize,
        messages: HashSet<i64>,
    },
    Broadcast {
        msg_id: usize,
        message: i64,
        idempotency_key: Option<String>,
    },
}

struct BroadcatHandler {
    label: String,
    id_counter: usize,
    node_ids: Vec<String>,
    topology: HashMap<String, Vec<String>>,
    messages: HashSet<i64>, // HashMap<String, i64>, //Vec<i64>,
}

impl BroadcatHandler {
    fn broadcast(&self, src: &String, msg_id: usize, value: i64) -> Vec<Message<Response>> {
        let neigbours = self.topology.get(&self.label).unwrap();

        let mut out = Vec::<Message<Response>>::new();

        // we need to acknowlege the receive of broadcast to whoever
        // send us the message
        out.push(Message::<Response> {
            src: self.label.clone(),
            dest: src.to_string(),
            body: Response::BroadcastOk {
                msg_id: self.id_counter,
                in_reply_to: msg_id,
            },
        });

        for neigbour in neigbours {
            if neigbour == src {
                continue;
            }

            out.push(Message {
                src: self.label.clone(),
                dest: neigbour.to_string(),
                body: Response::Broadcast {
                    msg_id,
                    message: value,
                    idempotency_key: None,
                },
            })
        }

        out
    }
}

impl Handle<Request, Response> for BroadcatHandler {
    fn new() -> Self {
        BroadcatHandler {
            label: String::new(),
            id_counter: 0,
            node_ids: Vec::new(),
            topology: HashMap::new(),
            messages: HashSet::new(),
        }
    }

    fn handle(&mut self, message: Message<Request>) -> anyhow::Result<Vec<Message<Response>>> {
        self.id_counter += 1;

        match message.body {
            Request::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.label = node_id;
                self.node_ids = node_ids;

                Ok(vec![Message::<Response> {
                    src: self.label.clone(),
                    dest: message.src,
                    body: Response::InitOk {
                        msg_id: self.id_counter,
                        in_reply_to: msg_id,
                    },
                }])
            }
            Request::Topology { msg_id, topology } => {
                self.topology = topology;

                Ok(vec![Message::<Response> {
                    src: self.label.clone(),
                    dest: message.src,
                    body: Response::TopologyOk {
                        msg_id: self.id_counter,
                        in_reply_to: msg_id,
                    },
                }])
            }
            Request::Broadcast {
                msg_id,
                message: value,
                ..
            } => {
                self.messages.insert(value);
                Ok(self.broadcast(&message.src, msg_id, value))
            }
            Request::Read { msg_id } => Ok(vec![Message::<Response> {
                src: self.label.clone(),
                dest: message.src,
                body: Response::ReadOk {
                    msg_id: self.id_counter,
                    in_reply_to: msg_id,
                    messages: self.messages.iter().copied().collect(),
                },
            }]),
            _ => Ok(vec![]),
        }
    }
}

fn main() -> anyhow::Result<()> {
    event_loop::<BroadcatHandler, Request, Response>()
}
