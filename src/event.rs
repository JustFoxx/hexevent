/// Event file
/// I - invoker type
/// R - additional return type
/// C - Callback arguments
/// Please know that this applies to everything in this file

pub struct EventKey<I, C> {
    invoker: I,
    callbacks: Vec<C>
}

impl<I, C> EventKey<I, C> {
    pub fn new(invoker: I) -> EventKey<I, C> {
        EventKey {
            invoker,
            callbacks: Vec::new(),
        }
    }

    pub fn on_event(&mut self, callback: C) {
        self.callbacks.push(callback);
    }

    pub fn invoke<'a, R>(&'a self) -> R
        where
            I: Fn(&'a [C]) -> R + 'a,
            R: 'a,
            C: 'a,
    {
        (self.invoker)(&self.callbacks)
    }
}
