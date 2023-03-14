pub struct EventKey<T,V> {
    pub invoker: Box<dyn Fn(&Vec<V>) -> T>,
    registry: Vec<V>,
}

pub trait Event<T, V> {
    fn on_event(&mut self, callback: V);
    fn new(invoker: Box<dyn Fn(&Vec<V>) -> T>) -> Self;
    fn invoker(&self) -> T;
}

impl<T,V> Event<T, V> for EventKey<T,V> {
    fn on_event(&mut self, callback: V) {
        self.registry.push(callback);
    }

    fn new(invoker: Box<dyn Fn(&Vec<V>) -> T>) -> Self {
        EventKey {
            invoker,
            registry: Vec::new(),
        }
    }

    fn invoker(&self) -> T {
        (self.invoker)(&self.registry)
    }
}