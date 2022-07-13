use std::fs::File;
use std::io;
use std::io::{Read, Write};

use rand::distributions::{Uniform, Distribution};

use tracing::error;

use crate::ARGS;

const BUFFER_SIZE: usize = 16384;


pub fn process_file(source_file: &mut File, key_file: &mut File, out_file: &mut File) -> Result<(), io::Error> {
	let mut source_buffer = [0; BUFFER_SIZE];
	let mut key_buffer = [0; BUFFER_SIZE];

	if ARGS.decrypt {
		loop {
			let source_bytes_read = source_file.read(&mut source_buffer)?;
			let key_bytes_read = key_file.read(&mut key_buffer)?;

			if source_bytes_read != key_bytes_read {
				error!("Amount of bytes read from source file different from amount of bytes read from key file! \
				Read {source_bytes_read} source bytes and {key_bytes_read} key bytes.");
				// panic!("source_bytes_read != key_bytes_read");
				std::process::exit(69);
			}

			if source_bytes_read == 0 {
				break;
			}

			xor_with_key(&mut source_buffer, &key_buffer, source_bytes_read);

			out_file.write_all(&source_buffer[..source_bytes_read])?;
		}
	} else {
		loop {
			let source_bytes_read = source_file.read(&mut source_buffer)?;

			if source_bytes_read == 0 {
				break
			}

			generate_key(&mut key_buffer, source_bytes_read);

			xor_with_key(&mut source_buffer, &key_buffer, source_bytes_read);

			out_file.write_all(&source_buffer[0..source_bytes_read])?;
			key_file.write_all(&key_buffer[0..source_bytes_read])?;
		}
	}


	Ok(())
}

fn xor_with_key(source: &mut [u8], key: &[u8], amount_source_bytes: usize) {
	for i in 0..amount_source_bytes {
		source[i] ^= key[i];
	}
}

/// Generates (part of) the key used for encrypting the file.
fn generate_key(key: &mut [u8], amount: usize) {
	let mut rng = rand::thread_rng();
	let uniform_distribution: Uniform<u8> = Uniform::new_inclusive(0, 255);
	for i in 0..amount {
		key[i] = uniform_distribution.sample(&mut rng);
	}
}

