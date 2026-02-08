hex2bin-fast v0.5.0

高速な16進数からバイナリへの変換ツール

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

基本的な使い方:
  hex2bin-fast <input.hex> <output.bin>

並列処理（推奨）:
  hex2bin-fast -p <input.hex> <output.bin>
  hex2bin-fast --parallel <input.hex> <output.bin>

スレッド数指定:
  hex2bin-fast -p --threads 8 <input.hex> <output.bin>

標準入出力:
  echo 48656C6C6F | hex2bin-fast - -
  cat input.hex | hex2bin-fast -p - output.bin

ヘルプ:
  hex2bin-fast --help

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

オプション:
  -p, --parallel        並列処理を有効化（大幅な高速化）
  --threads N           使用するスレッド数を指定
  -h, --help            ヘルプを表示

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

入力形式:
  - 16進数文字: 0-9, A-F, a-f
  - スペース、改行は自動的に無視
  - 大文字小文字どちらでも可

技術:
  - Rayonによるマルチスレッド並列処理
  - メモリマップドファイルI/O
  - 最適化されたLookup Table変換

作者: mei-sde
ライセンス: MIT License
詳細: https://github.com/mei-sde/hex2bin-fast

