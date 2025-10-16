use std::{collections::HashMap, sync::Arc, thread};

use parking_lot::Mutex;
use plugin_lib::types::{EventBus as Bus, Event};

#[derive(Clone)]
pub struct EventBus {
    handlers: Arc<Mutex<HashMap<(uuid::Uuid, String), Box<dyn FnMut(Event) + Send + Sync>>>>,
    event_queue: Arc<Mutex<Vec<Event>>>,
}

impl Bus for EventBus {
    fn emit(&mut self, event: Event) {
        self.event_queue.lock().push(event);
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
            event_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn process_events(&self) {
        let handlers = self.handlers.clone();
        let event_queue = self.event_queue.clone();
        thread::spawn(move || {
            loop {
                while let Some(event) = event_queue.lock().pop() {
                    let name = event.name().to_string();
                    for ((_, handler_name), handler) in handlers.lock().iter_mut() {
                        if handler_name == &name {
                            handler(event.clone());
                        }
                    }
                }
            }
        });
    }
}