use std::fmt::Debug;

pub enum EventState {
    Handled,
    Pending,
}

pub trait Event: Send + Debug {}
