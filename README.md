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
`-l, --length-of-vec N [default: 1000000]`

計算する配列の長さを指定する

`-r, --repeat-times N [default: 400000]`

計算量を調節するために，各スレッドが割り当てられた配列部分の和を計算する回数

`-n, --thread_num N [default: 1]`

計算に使用するスレッド数を指定する

`--task_size N [default: 1000]`

各スレッドが一度のタスクで計算する配列要素の数

## Features
### Flow
1. 配列を生成
2. 時間計測を開始
3. 配列を各スレッドが一度のタスクで計算する要素の数で分割
4. スレッドプール作成
5. 分割した各配列をワーカスレッドに渡し和を計算
6. ワーカスレッドは計算量を増加させるため5の計算を繰り返す
7. 計算の終了を待ち，各和の総和を計算
8. 計算時間を算出，出力
