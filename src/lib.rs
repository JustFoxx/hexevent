mod event;

#[cfg(test)]
mod tests {
    use crate::event::*;
    #[test]
    fn it_works() {
        let mut event_test = EventKey::new(Box::new(
                |entries: &Vec<&dyn Fn(i32)>| -> u32 {
                    let mut x = 1;
                    for entry in entries {
                        x*=3;
                        (entry)(x);
                    }
                    entries.len() as u32
                }
            ));
        let test = &|x: i32| {
            println!("Hello {}", x);
        };
        event_test.on_event(test);
        event_test.on_event(test);
        event_test.on_event(test);
        let result = event_test.invoker();
        assert_eq!(result, 3);
    }
}
