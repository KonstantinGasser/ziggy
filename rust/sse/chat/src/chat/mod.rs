use std::collections::HashMap;
use std::sync::Mutex;

use tokio::sync::broadcast::{Receiver, Sender};
use tracing::{error, info};

/*
* What should the chat app do?
*
* I want to launch two browser windows each connecting to the same shared app state.
*
* I want to see and create chats. A chat is a construct in which at least one other browser window
* different to my own is interacting (sending messages) and I can interact with.
*
* Therefore, the app needs to be able to create chat rooms with me and at least on other
* known/online person in it
*
*/

/// User represents an open connection
/// made through a browser window. Each
/// user must be uniquely identified by a user-handle
#[derive(Default)]
pub(crate) struct User {}

pub(crate) struct Hangout {
    pub(crate) users: Vec<User>,
    rx: Option<Receiver<String>>,
    tx: Option<Sender<String>>,
}

// State is the entire state of all online
// users and ongoing chat rooms.
#[derive(Default)]
pub(crate) struct State {
    online: Mutex<HashMap<usize, User>>,
    rooms: Mutex<HashMap<String, Hangout>>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn create_hangout(&self, name: &str) -> Option<Hangout> {
        self.rooms.lock().unwrap().insert(
            name.to_string(),
            Hangout {
                users: Vec::new(),
                rx: None,
                tx: None,
            },
        )
    }

    pub fn get_hangout_short(&self) -> Vec<String> {
        self.rooms
            .lock()
            .unwrap()
            .keys()
            .into_iter()
            .map(|key| key.to_owned())
            .collect()
    }

    pub fn init_hangout(&self, name: &str) -> Option<()> {
        let (tx, rx) = tokio::sync::broadcast::channel::<String>(16);

        let mut hangout = self.rooms.lock().unwrap();
        let Some(hangout) = hangout.get_mut(name) else {
            return None;
        };

        hangout.tx = Some(tx);
        hangout.rx = Some(rx);

        Some(())
    }

    pub fn connect_to_hangout(&self, name: &str) -> Option<Receiver<String>> {
        let hangout = self.rooms.lock().unwrap();

        let hangout = hangout.get(name).unwrap();

        let Some(ref rx) = hangout.rx else {
            return None;
        };

        Some(rx.resubscribe())
    }

    pub fn broadcast_to_hangout(&self, name: &str, msg: &str) -> Option<()> {
        let hangout = self.rooms.lock().unwrap();
        let hangout = hangout.get(name).unwrap();

        let Some(ref tx) = hangout.tx else {
            return None;
        };

        match tx.send(msg.to_string()) {
            Err(err) => {
                error!("Sending message to hangout \"{}\": {}", name, err);
                None
            }
            _ => Some(()),
        }
    }
}
