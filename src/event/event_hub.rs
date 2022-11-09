mod subscriber;

pub struct EventHub<'a> {
    subscribers: Vec<&'a dyn Subscriber>,
}