#![allow(dead_code, unused_imports)]

mod a;
mod aa;
mod path;
mod use_;

fn main() {
    a::item();
    a::b::item();

    aa::item();
    aa::b::item();
    aa::bb::item();
    aa::bb::c::item();

    {
        use rust_modules::*;

        item();
        aaa::item();
    }

    rust_modules::item();
    rust_modules::aaa::item();
    rust_modules::a::item();
    // error
    // rust_modules::aa::item();
}
