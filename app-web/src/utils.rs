use std::panic;

extern crate console_error_panic_hook;

pub fn set_panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook))
}
