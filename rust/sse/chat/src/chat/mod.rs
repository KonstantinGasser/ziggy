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
#[derive(Default, Debug, Clone)]
pub(crate) struct User {
    // pub(crate) id: String,
    pub(crate) id: String,
    pub(crate) handle: String,
}

pub(crate) type UserInfo = (String, String);

pub(crate) struct Hangout {
    pub(crate) users: Vec<User>,
    pub(crate) history: Vec<(i64, String)>,
    rx: Option<Receiver<Message>>,
    tx: Option<Sender<Message>>,
}

#[derive(Clone)]
pub(crate) enum Message {
    ChatMessage(String),
    UserJoin(String),
}

// State is the entire state of all online
// users and ongoing chat rooms.
#[derive(Default)]
pub(crate) struct State {
    online: Mutex<HashMap<String /*uuid::Uuid string */, User>>,
    rooms: Mutex<HashMap<String, Hangout>>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn claim_user_handle(&self, user_handle: &str) -> Option<String> {
        if self
            .online
            .lock()
            .unwrap()
            .values()
            .any(|user| user.handle == user_handle)
        {
            return None;
        }

        let mut online = self.online.lock().unwrap();

        let id = uuid::Uuid::new_v4().to_string();
        let _ = online.insert(
            id.clone(),
            User {
                id: id.clone(),
                handle: user_handle.to_string(),
            },
        );

        Some(id.clone())
    }

    pub fn create_hangout(&self, name: &str) -> Option<Hangout> {
        self.rooms.lock().unwrap().insert(
            name.to_string(),
            Hangout {
                users: Vec::new(),
                history: Vec::new(),
                rx: None,
                tx: None,
            },
        )
    }

    pub fn get_online_users(&self) -> Vec<UserInfo> {
        let Ok(online) = self.online.lock() else {
            return Vec::new();
        };

        online
            .values()
            .map(|user| (user.id.clone(), user.handle.clone()))
            .collect()
    }

    pub fn get_hangout_short(&self) -> Vec<String> {
        let Ok(rooms) = self.rooms.lock() else {
            return Vec::new();
        };

        rooms.keys().into_iter().map(|key| key.to_owned()).collect()
    }

    pub fn init_hangout(&self, name: &str) -> Option<()> {
        let mut hangout = self.rooms.lock().unwrap();
        let Some(hangout) = hangout.get_mut(name) else {
            return None;
        };

        if hangout.rx.is_some() && hangout.tx.is_some() {
            return Some(());
        }

        let (tx, rx) = tokio::sync::broadcast::channel::<Message>(16);
        hangout.tx = Some(tx);
        hangout.rx = Some(rx);

        Some(())
    }

    pub fn connect_to_hangout(
        &self,
        name: &str,
        user_id: &str,
    ) -> Option<(Receiver<Message>, Vec<UserInfo>)> {
        let users = self.online.lock().unwrap();
        let Some(user) = users.get(user_id) else {
            return None;
        };

        let mut hangout = self.rooms.lock().unwrap();
        let Some(hangout) = hangout.get_mut(name) else {
            return None;
        };

        hangout.users.push(user.clone());

        let Some(ref rx) = hangout.rx else {
            return None;
        };

        // TODO:
        // whoever is already in the hangout will not see that someone joined.
        // Hence, we would need to streame this particular event to all connected
        // connections
        let Some(ref tx) = hangout.tx else {
            return None;
        };

        match tx.send(Message::UserJoin(user_id.to_string())) {
            Err(err) => {
                error!("Sending message to hangout \"{}\": {}", name, err);
                None
            }
            _ => Some((
                rx.resubscribe(),
                hangout
                    .users
                    .iter()
                    .map(|u| (u.id.clone(), u.handle.clone()))
                    .collect(),
            )),
        }
    }

    pub fn broadcast_to_hangout(&self, name: &str, msg: &str) -> Option<()> {
        let hangout = self.rooms.lock().unwrap();
        let hangout = hangout.get(name).unwrap();

        let Some(ref tx) = hangout.tx else {
            return None;
        };

        match tx.send(Message::ChatMessage(msg.to_string())) {
            Err(err) => {
                error!("Sending message to hangout \"{}\": {}", name, err);
                None
            }
            _ => Some(()),
        }
    }
}
