# 変更履歴

## 2026年2月8日 - 並列化による高速化

### 性能結果（4GBファイル、16コアCPU）

| バージョン | 時間 | 高速化 |
|-----------|------|--------|
| v0.1.0（シングルスレッド初期版） | 33.2秒 | - |
| v0.2.0（並列化・初期実装） | 25.7秒 | 1.3倍 |
| v0.3.0（チャンク最適化） | 25.9秒 | 1.3倍 |
| **v0.4.0（フィルタリング並列化）** | **7.8秒** | **4.3倍** |

### スレッド数による性能変化（v0.4.0）

```
シングルスレッド: 26.9秒
--threads  2:     17.1秒 (1.6倍高速化)
--threads  4:     11.7秒 (2.3倍高速化)
--threads  8:      8.8秒 (3.1倍高速化)
--threads 12:      8.0秒 (3.4倍高速化)
--threads 16:      7.8秒 (3.4倍高速化)
```

---

## 実装された最適化

### ❌ 効果が低かった施策

#### 1. 小チャンクの並列化（v0.2.0）
```rust
hexstr.par_chunks(2).map(...).collect()
```
- **問題**: フィルタリングがシングルスレッドのまま（全処理の60%）
- **結果**: CPU使用率6-7%、並列処理は一瞬のみ
- **効果**: 1.3倍高速化

#### 2. チャンクサイズ拡大（v0.3.0）
```rust
hexstr.par_chunks(64 * 1024 * 1024 * 2).flat_map(...)
```
- **問題**: 依然としてフィルタリングがボトルネック
- **結果**: ほぼ変化なし
- **効果**: なし

---

### ✅ 効果が高かった施策（v0.4.0）

#### 1. **フィルタリングの並列化**（最重要）
```rust
input_bytes
    .par_chunks(16 * 1024 * 1024)  // 16MB単位で並列化
    .flat_map(|chunk| {
        // 各チャンクで独立して処理
        フィルタリング + 変換
    })
```
- **効果**: 最大のボトルネック（15秒）を並列化 → 4秒に短縮
- **CPU使用率**: 6-7% → 40-80%

#### 2. **UTF-8検証の省略**
```rust
// 変更前
File::open()?.read_to_string(&mut input)?

// 変更後
std::fs::read(&config.input_path)?
```
- **効果**: 不要なUTF-8検証コストを削減（約2秒短縮）

#### 3. **処理の一括化**
```rust
// 各チャンク内で即座に変換
for &b in chunk {
    if is_hex_digit(b) { hex_chars.push(b); }
}
for pair in hex_chars.chunks(2) {
    result.push(hex_to_byte(pair[0], pair[1]));
}
```
- **効果**: 中間の巨大Vec生成を回避、メモリコピー削減

#### 4. **Lookup Tableによる変換**
```rust
const HEX_LOOKUP: [u8; 256] = [...];

#[inline]
fn hex_to_byte(hi: u8, lo: u8) -> u8 {
    (HEX_LOOKUP[hi as usize] << 4) | HEX_LOOKUP[lo as usize]
}
```
- **効果**: `to_digit(16)`より2-3倍高速

#### 5. **適切なチャンクサイズ（16MB）**
- 4GBファイル ÷ 16MB = 約256チャンク
- 16コアで各コアが約16チャンクを処理
- 負荷分散とオーバーヘッドのバランスが最適

---

## 使用方法

```powershell
# シングルスレッド処理
.\hex2bin-fast.exe input.hex output.bin

# 並列処理（自動スレッド数）
.\hex2bin-fast.exe --parallel input.hex output.bin
.\hex2bin-fast.exe -p input.hex output.bin

# 並列処理（スレッド数指定）
.\hex2bin-fast.exe -p --threads 8 input.hex output.bin

# ヘルプ表示
.\hex2bin-fast.exe --help
```

---

## 今後の改善案

### メモリマップドファイルの使用
- `memmap2` crateでファイルI/Oを削減
- 予想効果: 7.8秒 → 5-6秒（さらに30%高速化）

### ストリーミング処理
- メモリ使用量をさらに削減
- 超大型ファイル（10GB以上）に対応
