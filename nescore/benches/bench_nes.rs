#![feature(test)]

extern crate test;

use test::Bencher;
use nescore::{Nes, Cartridge};

#[bench]
fn bench_emulate_frame(b: &mut Bencher) {
    let mut nes = Cartridge::from_path("tests/roms/nestest/nestest.nes")
                        .map(|cart| Nes::default().with_cart(cart)).unwrap();

    b.iter(|| nes.emulate_frame());
}

#[bench]
fn bench_nestest(b: &mut Bencher) {
    let mut nes = Cartridge::from_path("tests/roms/nestest/nestest.nes")
                        .map(|cart| Nes::default().with_cart(cart).entry(0xC000)).unwrap();

    // TODO: The PC needs to be reset before running again
    b.iter(|| nes.run_until(0xC66E));
}