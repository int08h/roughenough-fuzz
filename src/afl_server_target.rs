extern crate afl;
extern crate roughenough;
#[macro_use] extern crate slog;
extern crate slog_scope;
#[macro_use] extern crate log;
extern crate slog_term;
extern crate slog_stdlog;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::fs::OpenOptions;
use slog::Drain;
use roughenough::server::Server;

fn main() {
    let file = OpenOptions::new().create(true).write(true).open("my_log").unwrap();

    /*let decorator = slog_term::PlainSyncDecorator::new();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();*/


    info!("Starting fuzz binary!");

    let config = roughenough::config::make_config("ENV").unwrap();
    let server = AssertUnwindSafe(RefCell::new(Server::new(config)));

    // afl.rs alwauys aborts on panic,
    // so we don't care about unwind safety
    afl::fuzz(|bytes: &[u8]| {
        info!("Sending bytes in fuzz loop: {}", bytes.len());
        let mut borrow = server.borrow_mut();

        for chunk in bytes.chunks(1024) {
            borrow.send_to_self(chunk);
        }
        borrow.process_events();
    });
}
