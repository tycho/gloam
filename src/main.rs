//! Thin binary shim — all logic lives in the `gloam` library crate so that
//! dev tooling (the `cargo xtask` bundler) can reuse it.

fn main() {
    gloam::main();
}
