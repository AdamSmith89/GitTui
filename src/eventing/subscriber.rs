use crate::eventing::events::Event;

pub trait Subscriber {
    fn callback(&mut self, event: &Event);
}