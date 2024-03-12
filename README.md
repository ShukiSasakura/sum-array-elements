# sum-array-elements

## Overview
Wasm に対応した，配列の各要素の総和を計算するプログラム

## Requirement
+ Rust
+ Wasmer

### Install Rust
[Rust をインストール](https://www.rust-lang.org/ja/tools/install)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Install Wasmer
[Wasmer](https://github.com/wasmerio/wasmer)
```
curl https://get.wasmer.io -sSfL | sh
```
[WASIX](https://wasix.org/docs/language-guide/rust/installation)
```
cargo install cargo-wasix
```

## Usage
+ Rust native
```
cargo build --release
./target/release/sum-array-elements
```
+ Wasmer + WASIX
```
cargo wasix build --release
wasmer target/wasm32-wasmer-wasi/release/sum-array-elements.wasm
```
### Option
`-n N`
計算に使用するスレッド数を指定する

`-w N`
各スレッドが一度に計算する配列要素の数

## Features
### Flow
1. 配列を生成する
2. 配列を各スレッドが一度に計算する要素の数で分割する
3. 分割した部分の総和を各スレッドが求める
4. 計算量を増加させるため，3.を繰り返す
4. 計算の終了を待ち，各和の総和を求める
