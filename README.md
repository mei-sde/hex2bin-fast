# hex2bin-fast

高速な16進数文字列からバイナリファイルへの変換ツール

## 概要

hex2bin-fastは、16進数文字列をバイナリデータに変換するシンプルで高速なコマンドラインツールです。ファイルまたは標準入力から16進数文字列を読み込み、バイナリデータファイルまたは標準出力へ変換します。

## 特徴

- 🚀 高速な変換処理
- 📝 ファイルまたは標準入力/出力に対応
- 🧹 自動的に空白や改行などの不要な文字を除去
- ⚡ シンプルで使いやすいインターフェース

## インストール

### Cargoを使用してインストール

```bash
cargo install --path .
```

### ソースからビルド

```bash
git clone https://github.com/mei-sde/hex2bin-fast.git
cd hex2bin-fast
cargo build --release
```

ビルドされたバイナリは `target/release/hex2bin-fast` に配置されます。

## 使用方法

### 基本的な使い方

```bash
hex2bin-fast <input.hex> <output.bin>
```

### ファイルからファイルへの変換

```bash
hex2bin-fast input.hex output.bin
```

### 標準入力から標準出力への変換

```bash
echo "48656C6C6F" | hex2bin-fast - -
```

### ファイルから標準出力への変換

```bash
hex2bin-fast input.hex - | xxd
```

### 標準入力からファイルへの変換

```bash
cat input.hex | hex2bin-fast - output.bin
```

## 入力形式

16進数文字列は以下の形式をサポートします：

- スペース、タブ、改行は自動的に無視されます
- 大文字・小文字どちらでも可能です
- 純粋な16進数文字（0-9, A-F, a-f）のみが処理されます

### 入力例

```
48 65 6C 6C 6F 20 57 6F 72 6C 64
```

または

```
48656C6C6F20576F726C64
```

どちらも "Hello World" に変換されます。

## 必要要件

- Rust 1.56.0 以上

## ライセンス

MIT License

## 貢献

Issue報告やプルリクエストを歓迎します！

## 作者

mei-sde
