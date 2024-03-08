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
スレッド数（配列の分割数）を指定する

## Features
### Flow
1. 配列を生成する
2. 配列を与えられたスレッド数に分割する
3. 分割した部分の総和を各スレッドが求める
4. 各スレッドの計算の終了を待ち，各和の総和を求める
