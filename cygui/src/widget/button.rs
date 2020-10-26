use crate::widget::Widget;
use crate::{Event, Handler};
use cgmath::Vector2;
use cybuf::Drawable;
use cyio::Touch;
use std::sync::mpsc::Sender;
use std::{marker::PhantomData, sync::mpsc};

pub struct ButtonBuilder<T> {
    events: Option<ButtonEvent<T>>,
    position: Vector2<usize>,
    size: Vector2<usize>,
}

impl<T> ButtonBuilder<T> {
    pub fn new(position: Vector2<usize>, size: Vector2<usize>) -> Self {
        Self {
            events: None,
            position,
            size,
        }
    }

    pub fn with_events(mut self, events: ButtonEvent<T>) -> Self {
        self.events = Some(events);
        self
    }

    pub fn build<D>(self) -> Button<D, T>
    where
        D: Drawable,
        T: Clone,
    {
        Button {
            position: self.position,
            size: self.size,
            config: self.events.unwrap_or_default(),
            last_pos: Touch::None,
            tx: None,
            _ph_drawable: Default::default(),
            _ph_event: Default::default(),
        }
    }
}

pub struct ButtonEvent<T> {
    pub enter: Option<Event<T>>,
    pub leave: Option<Event<T>>,
    pub pressed: Option<Event<T>>,
    pub released: Option<Event<T>>,
    pub enter_second: Option<Event<T>>,
    pub leave_second: Option<Event<T>>,
    pub pressed_second: Option<Event<T>>,
    pub released_second: Option<Event<T>>,
}

impl<T> Default for ButtonEvent<T> {
    fn default() -> Self {
        Self {
            enter: None,
            leave: None,
            pressed: None,
            released: None,
            enter_second: None,
            leave_second: None,
            pressed_second: None,
            released_second: None,
        }
    }
}

enum Nearest {
    First,
    Second,
}

enum State {
    Enter,
    EnterSecond,
    Leave,
    LeaveSecond,
    Pressed,
    PressedSecond,
    Released,
    ReleasedSecond,
    None,
}

pub struct Button<D, T>
where
    D: Drawable,
    T: Clone,
{
    position: Vector2<usize>,
    size: Vector2<usize>,
    config: ButtonEvent<T>,
    last_pos: cyio::Touch,
    tx: Option<mpsc::Sender<Event<T>>>,
    _ph_drawable: PhantomData<D>,
    _ph_event: PhantomData<T>,
}

impl<D, T> Button<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn intersect(&self, pos: Vector2<usize>) -> bool {
        self.position.x < pos.x
            && pos.x < self.position.x + self.size.x
            && self.position.y < pos.y
            && pos.y < self.position.y + self.size.y
    }

    fn check_move(&self, last: Vector2<usize>, new: Vector2<usize>) -> State {
        match (self.intersect(last), self.intersect(new)) {
            (true, false) => State::Leave,
            (false, true) => State::Enter,
            _ => State::None,
        }
    }

    fn nearest(pos: Vector2<usize>, pos_1: Vector2<usize>, pos_2: Vector2<usize>) -> Nearest {
        let v1 = pos_1 - pos;
        let v2 = pos_2 - pos;
        let d1 = v1.x * v1.x + v1.y * v1.y;
        let d2 = v2.x * v2.x + v2.y * v2.y;
        match d1 < d2 {
            true => Nearest::First,
            false => Nearest::Second,
        }
    }

    fn send_event(&self, e: Option<Event<T>>) {
        if let Some(e) = e {
            self.tx.as_ref().unwrap().send(e).unwrap_or(());
        }
    }

    fn send(&self, s: State) {
        match s {
            State::Enter => self.send_event(self.config.enter.clone()),
            State::EnterSecond => self.send_event(self.config.enter_second.clone()),
            State::Leave => self.send_event(self.config.leave.clone()),
            State::LeaveSecond => self.send_event(self.config.leave_second.clone()),
            State::Pressed => self.send_event(self.config.pressed.clone()),
            State::PressedSecond => self.send_event(self.config.pressed_second.clone()),
            State::Released => self.send_event(self.config.released.clone()),
            State::ReleasedSecond => self.send_event(self.config.released_second.clone()),
            State::None => {}
        };
    }
}

impl<D, T> Widget<D, T> for Button<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn draw(&self, _: &mut D) {}
}

impl<D, T> Handler<T> for Button<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn attach(&mut self, tx: Sender<Event<T>>) {
        self.tx = Some(tx);
    }

    fn handle_event(&mut self, event: Event<T>) {
        match event {
            Event::IO(event) => match event {
                cyio::Event::TouchPressed(touch) => match touch {
                    Touch::One(pos) => {
                        // New touch
                        if self.intersect(pos) {
                            self.send(State::Pressed);
                        }
                    }
                    _ => {}
                },
                cyio::Event::TouchReleased => match self.last_pos {
                    Touch::One(pos) => {
                        // Touch released
                        if self.intersect(pos) {
                            self.send(State::Released);
                        }
                    }
                    _ => {}
                },
                cyio::Event::TouchMove(touch) => match touch {
                    Touch::One(pos) => match self.last_pos {
                        Touch::One(pos_last) => self.send(self.check_move(pos_last, pos)),
                        Touch::Two(pos_last_1, pos_last_2) => {
                            let (near, far) = match Self::nearest(pos, pos_last_1, pos_last_2) {
                                Nearest::First => (pos_last_1, pos_last_2),
                                Nearest::Second => (pos_last_2, pos_last_1),
                            };
                            // Move first touch
                            self.send(self.check_move(near, pos));
                            // Release second touch
                            if self.intersect(far) {
                                self.send(State::ReleasedSecond);
                            }
                        }
                        _ => {}
                    },
                    Touch::Two(pos_1, pos_2) => {
                        match self.last_pos {
                            Touch::One(last_pos) => {
                                let (near, far) = match Self::nearest(last_pos, pos_1, pos_2) {
                                    Nearest::First => (pos_1, pos_2),
                                    Nearest::Second => (pos_2, pos_1),
                                };
                                // Move first touch
                                self.send(self.check_move(last_pos, near));
                                // Press second touch
                                if self.intersect(far) {
                                    self.send(State::PressedSecond)
                                }
                            }
                            Touch::Two(pos_last_1, pos_last_2) => {
                                // Move first touch
                                self.send(self.check_move(pos_last_1, pos_1));
                                self.send(self.check_move(pos_last_2, pos_2));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
