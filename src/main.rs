#![allow(dead_code, unused_imports)]

mod a;
mod aa;
mod path;
mod use_;

use rust_modules_dependency;

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
    rust_modules::b::item();

    rust_modules_dependency::item();
}
