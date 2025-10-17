use std::{collections::HashMap, sync::Arc, thread};

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use plugin_lib::types::{EventBus as Bus, Event};

#[derive(Clone)]
pub struct EventBus {
    handlers: Arc<Mutex<HashMap<(uuid::Uuid, String), Box<dyn FnMut(Event) + Send + Sync>>>>,
    event_queue: Option<Sender<Event>>,
}

impl Bus for EventBus {
    fn emit(&mut self, event: Event) {
        if let Some(sender) = &self.event_queue {
            sender.send(event).unwrap();
        }
    }

    fn off(&mut self, id: uuid::Uuid) {
        self.handlers.lock().retain(|(handler_id, _), _| handler_id != &id);
    }

    fn on(&mut self, name: &str, handler: Box<dyn FnMut(Event) + Send + Sync>) -> uuid::Uuid {
        let id = uuid::Uuid::new_v4();
        self.handlers.lock().insert((id, name.to_string()), handler);
        id
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            event_queue: None,
        }
    }

    pub fn process_events(&mut self) {
        let handlers = self.handlers.clone();
        let (tx, rx) = crossbeam_channel::unbounded::<Event>();
        self.event_queue.replace(tx);
        thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                let name = event.name().to_string();
                for ((_, handler_name), handler) in handlers.lock().iter_mut() {
                    if handler_name == &name {
                        handler(event.clone());
                    }
                }
            }
        });
    }
}