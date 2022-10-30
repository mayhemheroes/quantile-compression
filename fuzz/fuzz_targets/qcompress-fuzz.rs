#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;
use q_compress::{auto_compress,auto_decompress,DEFAULT_COMPRESSION_LEVEL};

fn convert_to_u16(in: &[u8]) -> &[u16] {
      in.iter().map(|x| x as u16).collect()
}

fuzz_target!(|data: &[u8]| {
      auto_compress(convert_to_u16(in), DEFAULT_COMPRESSION_LEVEL);
});
