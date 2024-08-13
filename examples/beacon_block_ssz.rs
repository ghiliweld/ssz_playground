use ssz::{Decode, Encode};
use ssz_types::SignedBeaconBlock;
use std::fs;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[inline(never)]
fn decode(bytes: &Vec<u8>) -> SignedBeaconBlock {
    SignedBeaconBlock::from_ssz_bytes(&bytes).unwrap()
}

#[inline(never)]
fn encode(beacon_block: &SignedBeaconBlock) -> Vec<u8> {
    beacon_block.as_ssz_bytes()
}

fn main() {
    // prepare all ssz data
    // from (Deneb) slot: 9717504
    let bytes: Vec<u8> = fs::read("beacon-block.ssz").unwrap();
    let beacon_block = SignedBeaconBlock::from_ssz_bytes(&bytes).unwrap();

    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-block.json")
        .trim_backtraces(None)
        .build();

    // run ssz tests
    decode(&bytes);
    // decode_spec_test(&spec_bytes);
    // encode(&beacon_block);
    // encode_spec_test(&spec_block);
}
