pub trait Handler<T: Clone> {
    fn handle_event(&mut self, event: T);
}
