#[macro_use] extern crate afl;
extern crate roughenough;

use std::fs::File;
use std::io::prelude::*;

use std::env;

fn main() {
    let args = env::args();

    if args.count() == 2 as usize {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename).unwrap();
        let size = file.metadata().unwrap().len();
    
        let mut contents = Vec::with_capacity(size as usize);
        file.read_to_end(&mut contents).unwrap();
    
        let _ = roughenough::RtMessage::from_bytes(&contents);
    } else {
        fuzz!(|bytes: &[u8]| {
            let _ = roughenough::RtMessage::from_bytes(bytes);
        });
    }
}
