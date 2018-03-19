extern crate afl;
extern crate roughenough;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = roughenough::RtMessage::from_bytes(&bytes);
    });
}
