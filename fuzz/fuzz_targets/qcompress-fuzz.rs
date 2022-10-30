#![no_main]

extern crate libfuzzer_sys;
extern crate q_compress;

use libfuzzer_sys::fuzz_target;
use q_compress::{auto_compress,auto_decompress,DEFAULT_COMPRESSION_LEVEL};

fuzz_target!(|data: &[u8]| {
      auto_compress(data.to_vec().iter().map(|x| x as &u16).collect(), DEFAULT_COMPRESSION_LEVEL);
});
