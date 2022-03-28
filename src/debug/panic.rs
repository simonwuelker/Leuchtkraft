//! Custom panic behaviour

use std::panic;

pub fn init() {
    panic::set_hook(Box::new(|panic_info| {
        println!("The interpreter unexpectedly panicked. This is a bug. Please file an issue at https://github.com/Wuelle/Leuchtkraft/issues");
        if let Some(location) = panic_info.location() {
            println!("panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        }
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("Info: {s:?}");
        }
    }));
}
