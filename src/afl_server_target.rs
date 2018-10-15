extern crate afl;
extern crate roughenough;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use roughenough::server::Server;

fn main() {
    let config = roughenough::config::make_config("ENV").unwrap();

    // We don't care about unwind safety,
    // afl.rs alwauys aborts on panic,
    let server = AssertUnwindSafe(RefCell::new(Server::new(config)));

    afl::fuzz(|bytes: &[u8]| {
        let mut borrow = server.borrow_mut();

        // Split fuzzer input into multiple packets
        // to test batching functionality
        for chunk in bytes.chunks(1024) {
            borrow.send_to_self(chunk);
        }
        borrow.process_events();
    });
}
