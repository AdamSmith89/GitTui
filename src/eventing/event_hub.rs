use crate::eventing::{events::Event, subscriber::*};

pub struct EventHub {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl EventHub {
    pub fn new() -> Self {
        EventHub { subscribers: Vec::new() }
    }

    pub fn publish(&mut self, event: Event) {
        for subscriber in &mut self.subscribers {
            subscriber.callback(&event);
        }
    }

    pub fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }
}
