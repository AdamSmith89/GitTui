pub trait Subscriber {
    fn callback(&self, event: &Event);
}