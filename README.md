# hexevent
Simple callback based event system.
## Example Usage
```rust
use hexevent::event::*;
fn main() {
    let mut key = EventKey::new(|callbacks: &[&str]| {
        for callback in callbacks {
            println!("{}", callback);
        }
    });
    key.on_event("Hello");
    key.on_event("World");
    key.invoke();
}
```