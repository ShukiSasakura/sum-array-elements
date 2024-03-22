# sum-array-elements

## Overview
Wasm に対応した，配列の各要素の総和を計算するプログラム

## Requirement
+ Rust
+ Wasmer + WASIX

### Install Rust
[Rust をインストール](https://www.rust-lang.org/ja/tools/install)
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Install Wasmer + WASIX
[Wasmer](https://github.com/wasmerio/wasmer)
```
$ curl https://get.wasmer.io -sSfL | sh
```
[WASIX](https://wasix.org/docs/language-guide/rust/installation)
```
$ cargo install cargo-wasix
```

## Usage
+ Rust native
```
$ cargo build --release
$ ./target/release/sum-array-elements
```
+ Wasmer + WASIX
```
$ cargo wasix build --release
$ wasmer target/wasm32-wasmer-wasi/release/sum-array-elements.wasm
```
### Option
`-l, --length-of-vec L [default: 1000000]`

計算する配列の長さを指定する

`-c, --chunk_size M [default: 1000]`

各スレッドが一度に計算する配列要素の数(chunk)

`-t, --thread_num N [default: 1]`

計算に使用するスレッド数を指定する

`-r, --repeat-times P [default: 400000]`

計算量を調節するために，各スレッドが割り当てられた chunk の和を計算する回数

### Example
+ Rust native
```
$ ./target/release/sum-array-elements -l 1000000 -c 1000 -t 4 -r 100000
```

+ Wasmer + WASIX
```
$ wasmer target/wasm32-wasmer-wasi/release/sum-array-elements.wasm -- -l 1000000 -c 1000 -t 4 -r 100000
```

## Features
### Flow
1. 長さ L の計算用配列を生成する
2. 時間計測を開始する
3. 計算用配列を大きさ M で分割する
4. N 個のワーカスレッドを持つスレッドプールを作成する
5. メインスレッドは chunk の総和を計算するタスクを配列ごとにワーカスレッドに渡す
6. ワーカスレッドは chunk の総和を計算する
7. ワーカスレッドは計算量を増加させるため6の計算を P 回繰り返す
8. メインスレッドはワーカスレッドの計算の終了を待ち，各和の総和を計算する
9. 計算時間を算出，出力する
