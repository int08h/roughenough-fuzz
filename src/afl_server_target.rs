extern crate afl;
extern crate roughenough;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use mio::Events;

use roughenough::server::Server;

fn main() {
    // Don't conflict with a real server running on port 8687
    let config = roughenough::config::MemoryConfig::new(8687);

    // We don't care about unwind safety,
    // afl.rs always aborts on panic,
    let server = AssertUnwindSafe(RefCell::new(Server::new(Box::new(config))));

    let mut events = Events::with_capacity(1024);

    afl::fuzz(|bytes| {
        let mut borrow = server.borrow_mut();

        // Split fuzzer input into multiple packets
        // to test batching functionality
        for chunk in bytes.chunks(1024) {
            borrow.send_to_self(chunk);
        }

        borrow.process_events(&mut events);
    });
}
