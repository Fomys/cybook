use std::fmt::Debug;

pub enum EventState {
    Handled,
    Pending,
}

#[derive(Clone, Debug)]
pub enum Event<T> {
    IO(cyio::Event),
    User(T),
}
