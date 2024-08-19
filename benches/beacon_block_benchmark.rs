use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ssz::{Decode, Encode};
use ssz_types::SignedBeaconBlock;

use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let block_bytes: Vec<u8> = fs::read("ssz-blocks/beacon-block.ssz").unwrap();
    let beacon_block = SignedBeaconBlock::from_ssz_bytes(&block_bytes).unwrap();

    c.bench_function("SignedBeaconBlock decode", |b| {
        b.iter(|| SignedBeaconBlock::from_ssz_bytes(black_box(&block_bytes)).unwrap())
    });

    c.bench_function("SignedBeaconBlock encode", |b| {
        b.iter(|| beacon_block.as_ssz_bytes())
    });

    c.bench_function("SignedBeaconBlock decode + encode", |b| {
        b.iter(|| {
            SignedBeaconBlock::from_ssz_bytes(black_box(&block_bytes))
                .unwrap()
                .as_ssz_bytes()
        })
    });

    // let state_bytes: Vec<u8> = fs::read("beacon-state.ssz").unwrap();
    // let beacon_state = BeaconState::from_ssz_bytes(&state_bytes).unwrap();

    // c.bench_function("BeaconState decode", |b| {
    //     b.iter(|| BeaconState::from_ssz_bytes(black_box(&state_bytes)).unwrap())
    // });

    // c.bench_function("BeaconState encode", |b| {
    //     b.iter(|| beacon_state.as_ssz_bytes())
    // });

    // c.bench_function("BeaconState decode + encode", |b| {
    //     b.iter(|| {
    //         BeaconState::from_ssz_bytes(black_box(&state_bytes))
    //             .unwrap()
    //             .as_ssz_bytes()
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
