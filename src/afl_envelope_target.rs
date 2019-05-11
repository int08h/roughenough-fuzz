#[macro_use] extern crate afl;
extern crate roughenough;

extern crate hex;

use std::fs::File;
use std::io::prelude::*;

use roughenough::kms::{EnvelopeEncryption, KmsProvider, KmsError};

use std::env;
use afl::fuzz;

struct NoOpKms;

impl KmsProvider for NoOpKms {
    fn encrypt_dek(&self, _ : &Vec<u8>) -> Result<Vec<u8>, KmsError> {
        unimplemented!()
    }

    fn decrypt_dek(&self, encrypted_dek: &Vec<u8>) -> Result<Vec<u8>, KmsError> {
        Ok(encrypted_dek.clone())
    }
}

fn main() {
    let args = env::args();
    let kms = NoOpKms {};

    if args.count() == 2 as usize {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename).unwrap();
        let size = file.metadata().unwrap().len();
    
        let mut contents = Vec::with_capacity(size as usize);
        file.read_to_end(&mut contents).unwrap();
    
        match EnvelopeEncryption::decrypt_seed(&kms, &contents) {
            Ok(seed) => println!("seed {}", hex::encode(seed)),
            Err(err) => println!("error {:?}", err)
        }
    } else {
        fuzz(|bytes: &[u8]| {
            let _ = EnvelopeEncryption::decrypt_seed(&kms, &Vec::from(bytes));
        });
    }
}
