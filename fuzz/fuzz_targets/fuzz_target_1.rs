#![no_main]
#[macro_use] extern crate libfuzzer_sys;

extern crate roughenough;

fuzz_target!(|data: &[u8]| {
     let _ = roughenough::RtMessage::from_bytes(data);
});
