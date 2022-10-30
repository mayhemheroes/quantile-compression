#![no_main]

extern crate libfuzzer_sys;
extern crate q_compress;

use libfuzzer_sys::fuzz_target;
use q_compress::{auto_compress,auto_decompress,DEFAULT_COMPRESSION_LEVEL};

fn convert_to_u16(data: &[u8]) -> [u16] {
      data.iter().map(|x| x as u16).collect()
}

fuzz_target!(|data: &[u8]| {
      auto_compress(&convert_to_u16(data), DEFAULT_COMPRESSION_LEVEL);
});
