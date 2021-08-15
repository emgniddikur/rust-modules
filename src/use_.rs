mod a {
    pub mod b {
        pub fn item() {}

        pub struct C {}
    }
}

// 関数の場合は親モジュールまでを書く（どこで定義されているかが明らかになるから）
use a::b;
// その他のitemはフルパスを書く（理由は不明）
use a::b::C;

use std::io::{self, Write};
