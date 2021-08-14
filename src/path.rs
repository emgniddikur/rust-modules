mod a {
    fn private_item() {}

    pub fn public_item() {}

    mod b {
        fn item() {
            // 子孫モジュールのitemは祖先モジュールのprivate itemにアクセスできる
            super::private_item();
        }
    }
}

fn item() {
    // 兄弟モジュールはpubが付いていなくてもアクセスできるが、その子孫モジュールのprivate itemにはアクセスできない

    // error
    // a::private_item();

    a::public_item();
}
