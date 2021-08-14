extern crate rust_modules;

mod a;
mod aa;

use rust_modules::*;

fn main() {
    a::item();
    a::b::item();

    aa::item();
    aa::b::item();
    aa::bb::item();
    aa::bb::c::item();

    // rust_modules
    item();
    aaa::item();

    rust_modules::item();
    rust_modules::aaa::item();
    rust_modules::a::item();
    // error
    // rust_modules::aa::item();
}
