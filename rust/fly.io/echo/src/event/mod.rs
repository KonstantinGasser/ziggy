use std::io::{BufRead, Write};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: T,
}

pub trait Handle<Request, Response> {
    fn handle(&mut self, message: Message<Request>) -> anyhow::Result<Message<Response>>;
}

pub fn stream<Request, Response>(mut handler: impl Handle<Request, Response>) -> anyhow::Result<()>
where
    Request: DeserializeOwned + Send + 'static,
    Response: Serialize,
{
    // TODO: each node will receive an inital "init" message
    // from maelstrom. Would make sense to parse it beforehand
    // and construct the node?
    let (tx, rx) = std::sync::mpsc::channel::<Message<Request>>();

    let read_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let stdin = std::io::stdin().lock();
        let lines = stdin.lines();

        // do the reading and parsing the the stdin messages
        for line in lines {
            let line = line.context("reading from stdin failed")?;

            let message: Message<Request> =
                serde_json::from_str(&line).context("parsing stdin to Message<Request>")?;

            tx.send(message).unwrap();
        }

        Ok(())
    });

    let mut stdout = std::io::stdout().lock();
    // read from channel of Message<T> and process them
    for message in rx.into_iter() {
        let response = handler
            .handle(message)
            .context("handler unable to process Message<Request>")?;

        serde_json::to_writer(&mut stdout, &response)
            .context("writing to stdout of Message<Response> failed")?;

        stdout
            .write(b"\n")
            .context("writing new line to stdout after write of Message<Response> failed")?;
    }

    read_thread
        .join()
        .expect("something failed during reading from stdin")?;
    Ok(())
}
