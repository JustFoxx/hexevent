/// Event file
/// I - return type of the invoker
/// V - type of the events
/// Please know that this applies to everything in this file


pub struct EventKey<I,V> {
    pub invoker: Box<dyn Fn(&Vec<V>) -> I>,
    registry: Vec<V>,
}

pub trait Event<I, V> {
    fn on_event(&mut self, callback: V);
    fn new(invoker: Box<dyn Fn(&Vec<V>) -> I>) -> Self;
    fn invoker(&self) -> I;
}

impl<I,V> Event<I, V> for EventKey<I,V> {
    fn on_event(&mut self, callback: V) {
        self.registry.push(callback);
    }

    fn new(invoker: Box<dyn Fn(&Vec<V>) -> I>) -> Self {
        EventKey {
            invoker,
            registry: Vec::new(),
        }
    }

    fn invoker(&self) -> I {
        (self.invoker)(&self.registry)
    }
}