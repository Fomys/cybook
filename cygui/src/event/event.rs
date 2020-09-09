pub enum EventState {
    Handled,
    Pending,
}

pub struct IOEvent {
    event: cyio::Event,
}

impl Event for IOEvent {}

impl From<cyio::Event> for IOEvent {
    fn from(event: cyio::Event) -> Self {
        Self { event }
    }
}

pub trait Event {}
