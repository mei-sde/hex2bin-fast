# hex2bin-fast

**超高速な16進数文字列からバイナリファイルへの変換ツール**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org/)

## 概要

hex2bin-fastは、16進数文字列をバイナリデータに高速変換するコマンドラインツールです。
**マルチスレッド並列処理**と**メモリマップドファイルI/O**により、大容量ファイルを驚異的な速度で処理します。

## 特徴

- 🚀 **超高速処理** - マルチコア並列処理で最大5倍以上高速化
- 💾 **メモリ効率的** - メモリマップドファイルで大容量ファイルも快適に処理
- ⚙️ **柔軟な設定** - スレッド数を自由に指定可能
- 📝 **標準I/O対応** - ファイルまたは標準入力/出力に対応
- 🧹 **自動整形** - 空白や改行などの不要な文字を自動除去
- ⚡ **シンプル** - 使いやすいコマンドラインインターフェース

## インストール

### ソースからビルド

```bash
git clone https://github.com/mei-sde/hex2bin-fast.git
cd hex2bin-fast
cargo build --release
```

ビルドされたバイナリは `target/release/hex2bin-fast.exe` に配置されます。

## 使用方法

### 基本的な使い方

```bash
# シングルスレッド処理
hex2bin-fast input.hex output.bin

# 並列処理（推奨）- 自動スレッド数
hex2bin-fast --parallel input.hex output.bin
hex2bin-fast -p input.hex output.bin

# 並列処理 - スレッド数指定
hex2bin-fast -p --threads 8 input.hex output.bin
```

### オプション

```
OPTIONS:
  -p, --parallel        並列処理を有効化（大幅な高速化）
  --threads N           使用するスレッド数を指定（デフォルト: CPU自動）
  -h, --help            ヘルプを表示
```

### 使用例

#### ファイルからファイルへの高速変換

```bash
# 並列処理で最速変換
hex2bin-fast -p input.hex output.bin
```

#### 標準入力から標準出力への変換

```bash
echo "48656C6C6F" | hex2bin-fast - -
```

#### パイプラインでの使用

```bash
cat input.hex | hex2bin-fast -p - output.bin
hex2bin-fast -p input.hex - | xxd
```

## 入力形式

16進数文字列は以下の形式をサポートします：

- **自動クリーニング**: スペース、タブ、改行は自動的に無視
- **大文字小文字**: どちらでも処理可能
- **純粋な16進数**: 0-9, A-F, a-f のみを処理

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
- Windows / Linux / macOS

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照

## 変更履歴

詳細な変更履歴と最適化の詳細は [HISTORY.md](HISTORY.md) を参照してください。

## 作者

mei-sde
