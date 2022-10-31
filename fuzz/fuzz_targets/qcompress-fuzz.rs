#![no_main]

extern crate libfuzzer_sys;
extern crate q_compress;

use libfuzzer_sys::fuzz_target;
use q_compress::Compressor;

fuzz_target!(|data: &[u8]| {
      let ivec = Vec::from(data);
      let uvec: Vec<u16> = ivec.iter().map(|x| *x as u16).collect();
      let mut compressor = Compressor::<u16>::default();
      let bytes = compressor.simple_compress(&uvec);
});
