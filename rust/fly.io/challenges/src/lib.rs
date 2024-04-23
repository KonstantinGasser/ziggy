use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: T,
}

impl<T> Message<T>
where
    T: Serialize,
{
    fn responed(&self, w: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_writer(w, self).context("writing to stdout of Message<Response> failed")
    }

    fn write(&self, w: &mut std::io::StdoutLock) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *w, self)
            .context("writing to stdout of Message<Response> failed")?;

        w.write(b"\n")
            .context("writing new line to stdout after write of Message<Response> failed")?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Init {
    msg_id: usize,
    node_id: String,
    node_ids: Vec<String>,
}

pub trait Handle<Request, Response> {
    fn new(label: &str) -> Self;
    fn handle(&mut self, message: Message<Request>) -> anyhow::Result<Vec<Message<Response>>>;
}

pub fn event_loop<H, Request, Response>() -> anyhow::Result<()>
where
    H: Handle<Request, Response>,
    Request: DeserializeOwned + Send + 'static,
    Response: Serialize,
{
    let stdin = std::io::stdin().lock();
    let init_line = stdin
        .lines()
        .next()
        .expect("first line not provied. First line required as init message");
    let init_line = init_line.expect("reading init line from stdin failed");

    let init_msg: Message<Init> =
        serde_json::from_str(&init_line).context("failed to deserialize init message")?;

    if let Err(_) = init_msg.responed(&mut std::io::stdout().lock()) {
        panic!("sending of init_ok message failed")
    }

    let mut handler = H::new(&init_msg.body.node_id);
    let mut stdout = std::io::stdout().lock();

    for line in std::io::stdin().lock().lines() {
        let line = line.context("reading from stdin failed")?;

        let message: Message<Request> =
            serde_json::from_str(&line).context("parsing stdin to Message<Request>")?;

        let responses = handler
            .handle(message)
            .context("handler unable to process Message<Request>")?;

        for response in responses {
            response
                .write(&mut stdout)
                .context("writing new line to stdout after write of Message<Response> failed")?;
        }
    }

    Ok(())
}
