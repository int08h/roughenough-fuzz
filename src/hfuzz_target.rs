#[macro_use] extern crate honggfuzz;

extern crate roughenough;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
			let _ = roughenough::RtMessage::from_bytes(data);
        });
    }
}
