mod event;
#[cfg(test)]
mod tests {
    use crate::events::*;
    fn test<>(mut x: i32,y: i32, entries: &[&dyn Fn(i32)]) -> i32 {
        for entry in entries {
            x *= 3;
            (entry)(x * y);
        }
        x * y
    }
    fn invoker<'a>(entries: &'a [&'a dyn Fn(i32)]) -> impl Fn(i32) -> i32 + 'a {
        move |y|test(1,y,entries)
    }
    #[test]
    fn it_works() {
        let mut event_test = EventKey::new(invoker);
        let test: &dyn Fn(i32) = &|x| {
            println!("Hello {}", x);
        };
        event_test.on_event(test);
        event_test.on_event(test);
        event_test.on_event(test);
        let result = event_test.invoke()(10);
        assert_eq!(result, 270);
    }
}
