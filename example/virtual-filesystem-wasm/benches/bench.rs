#![feature(test)]

extern crate test;
extern crate virtual_filesystem_wasm;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = virtual_filesystem_wasm::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}

