use alloc::{string::String, sync::Arc, vec::Vec};
use hashbrown::HashMap;
use spin::Mutex;

type EventCallback = Arc<dyn Fn(&[&str]) + Send + Sync>;

pub struct EventEmitter {
    _events: Mutex<HashMap<String, Vec<EventCallback>>>,
    _max_listeners: usize,
}

impl EventEmitter {
    pub fn new() -> Self {
        return EventEmitter {
            _events: Mutex::new(HashMap::new()),
            _max_listeners: usize::MAX,
        };
    }

    pub fn on(&mut self, event: &str, function: EventCallback) {
        let mut events = self._events.lock();
        let callbacks = events.entry(String::from(event)).or_insert(Vec::new());

        callbacks.push(function);
    }

    pub fn emit(&mut self, event: &str, args: &[&str]) {
        let events = self._events.lock();

        if let Some(callbacks) = events.get(event) {
            for callback in callbacks {
                let callback_clone = callback.clone();

                callback_clone(args);
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref EVENT_EMITTER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}
