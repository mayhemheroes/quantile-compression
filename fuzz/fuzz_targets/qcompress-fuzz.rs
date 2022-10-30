#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;
use q_compress::{auto_compress,auto_decompress,DEFAULT_COMPRESSION_LEVEL};


fuzz_target!(|data: &[u8]| {
      auto_compress(data, DEFAULT_COMPRESSION_LEVEL);
});
