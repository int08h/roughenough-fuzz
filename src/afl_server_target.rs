#[macro_use] extern crate afl;
extern crate roughenough;
extern crate log;
extern crate simple_logger;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use roughenough::server::Server;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = roughenough::config::make_config("ENV").unwrap();
    let server = AssertUnwindSafe(RefCell::new(Server::new(config)));
  
    // afl.rs alwauys aborts on panic,
    // so we don't care about unwind safety
    afl::fuzz(|bytes: &[u8]| {
        let mut borrow = server.borrow_mut();
        borrow.send_to_self(bytes);
        borrow.process_events();
    });
}
