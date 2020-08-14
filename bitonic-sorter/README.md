# bitonic-sorter

[バイトニックソート](https://ja.wikipedia.org/wiki/%E3%83%90%E3%82%A4%E3%83%88%E3%83%8B%E3%83%83%E3%82%AF%E3%82%BD%E3%83%BC%E3%83%88) をするモジュール.

本三章の写経 (+ Rustfmt/Clippy)

## モジュール

- `first`: `u32` のみ対応
- `second`: `T: Ord` に対応 (文字列など)
- `third`: 比較関数をクロージャで渡す
- `fourth`: 並列実行

## ベンチマーク

`cargo run --release --example benchmark -- 26` で並列実行のパフォーマンス差分測定
