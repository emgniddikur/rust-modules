# module system

パッケージ　 → 　クレート　 → 　モジュール

## クレート

バイナリかライブラリ

クレートルートはコンパイルの起点であり、クレートのルートモジュールを構成する

## パッケージ

Cargo.toml がある

ライブラリクレートは 0 つか 1 つ

バイナリクレートは 0 つ以上

1 つ以上のクレートを含める必要がある

## main.rs, lib.rs, bin/

main.rs、lib.rs は `crate` というモジュール名になる

### main.rs

main.rs はパッケージと同名のバイナリクレートのクレートルート

### lib.rs

lib.rs はパッケージと同名のライブラリクレートのクレートルート

### bin/

bin/にファイルを置くことで、それぞれのファイルが別々のバイナリクレートになる

## path

兄弟モジュールは`pub`が付いていなくてもアクセスできるが、その子孫モジュールの private item にはアクセスできない

祖先モジュールの item は子孫モジュールの private item にアクセスできないが、子孫モジュールの item は祖先モジュールの private item にアクセスできる

### absolute path

クレート名か`crate`から始まる

同じクレートの場合は`crate`を使うことができる

### relative path

`self`, `super`, 現在のモジュールから始まる

## use

モジュールをスコープに持ち込む

### use のパスをどこまで書くか

- 関数の場合は親モジュールまでを書く（どこで定義されているかが明らかになるから）
- その他の item はフルパスを書く（理由は不明）

### pub use

re-exporting

定義しやすい構造を呼び出しやすい構造に変換する

### glob 演算子

```rust
use std::collections::*;
```

- テストの際に、テスト対象の item を tests モジュールに持ち込むために使われる
- プレリュードパターンの一部として使われる

## Release Profiles

コンパイルのオプション一覧

[https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)

### dev profile

開発用

### release profile

リリース用

## ドキュメンテーションコメント

````rust
/// Greets to the name given.
///
/// # Examples
///
/// ```
/// let name = "John Doe";
///
/// assert_eq!(rust_modules::greet("John Doe"), "Hello, John Doe!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
````

```bash
$ cargo doc
$ cargo doc --open
```

[https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html#役に立つドキュメンテーションコメントを行う](https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html#%E5%BD%B9%E3%81%AB%E7%AB%8B%E3%81%A4%E3%83%89%E3%82%AD%E3%83%A5%E3%83%A1%E3%83%B3%E3%83%86%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3%E3%82%B3%E3%83%A1%E3%83%B3%E3%83%88%E3%82%92%E8%A1%8C%E3%81%86)

### よく使われるセクション

- Examples
- Panics
- Errors
- Safety

### Documentation comments within items

クレートやモジュール内部の item に doc コメントを書く場合は`//!`を使う

## ワークスペース

複数のパッケージをまとめたもの。ライブラリクレートを複数に分割したい場合に使う。

ワークスペースの最上位階層にだけ Cargo.lock が存在することからもわかるように、ワークスペース内の全クレートが、それが依存している全てのクレートを同じバージョンで使用することになる

### 実行用のコマンド

```bash
$ cargo run -p binary
```

### 個別でテストする場合のコマンド

```bash
$ cargo test -p binary
```

### crates.io に公開する場合

ワークスペースのクレートを crates.io に公開する場合は、各クレートに対して`cargo publish`する必要がある

## 参考

[https://doc.rust-jp.rs/book-ja/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html](https://doc.rust-jp.rs/book-ja/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

[https://doc.rust-jp.rs/book-ja/ch14-00-more-about-cargo.html](https://doc.rust-jp.rs/book-ja/ch14-00-more-about-cargo.html)

[https://qiita.com/skitaoka/items/753a519d720a1ccebb0d](https://qiita.com/skitaoka/items/753a519d720a1ccebb0d)
