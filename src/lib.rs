mod event;

#[cfg(test)]
mod tests {
    use crate::event::*;
    #[test]
    fn it_works() {
        fn invoker(entries: &Vec<&dyn Fn(i32)>) -> impl Fn(i32) -> i32 {
            let mut x = 1;
            for entry in entries {
                x *= 3;
                (entry)(x);
            };
            move |y| {
                x * y
            }
        }

        let mut event_test = EventKey::new(Box::new(
                invoker,
        ));
        let test = &|x: i32| {
            println!("Hello {}", x);
        };
        event_test.on_event(test);
        event_test.on_event(test);
        event_test.on_event(test);
        let result = event_test.invoker()(2);
        assert_eq!(result, 54);
    }
}
