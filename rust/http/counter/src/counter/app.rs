use std::{
    sync::{Arc, Mutex},
    usize,
};

#[derive(Clone, Default)]
pub struct App {
    counter: Arc<Mutex<usize>>,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn get_count(&self) -> usize {
        *self.counter.lock().unwrap()
    }

    pub fn increment(&self) -> usize {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        *counter
    }
    pub fn decrement(&self) -> Option<usize> {
        let mut counter = self.counter.lock().unwrap();
        if *counter == 0 {
            return None;
        }
        *counter -= 1;
        Some(*counter)
    }
}
