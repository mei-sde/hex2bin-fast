use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter};
use rayon::prelude::*;

// 高速な16進数変換用のルックアップテーブル
const HEX_LOOKUP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   255, 255, 255, 255, 255, 255,  // 0-9
    255, 10,  11,  12,  13,  14,  15,  255, 255, 255, 255, 255, 255, 255, 255, 255,  // A-F
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 10,  11,  12,  13,  14,  15,  255, 255, 255, 255, 255, 255, 255, 255, 255,  // a-f
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

#[inline]
fn hex_to_byte(hi: u8, lo: u8) -> u8 {
    (HEX_LOOKUP[hi as usize] << 4) | HEX_LOOKUP[lo as usize]
}

struct Config {
    input_path: String,
    output_path: String,
    parallel: bool,
    threads: Option<usize>,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    
    let mut parallel = false;
    let mut threads = None;
    let mut input_path = None;
    let mut output_path = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--parallel" | "-p" => {
                parallel = true;
                i += 1;
            }
            "--threads" => {
                if i + 1 >= args.len() {
                    return Err("--threads requires a number".to_string());
                }
                threads = Some(args[i + 1].parse::<usize>()
                    .map_err(|_| "Invalid thread count".to_string())?);
                i += 2;
            }
            "--help" | "-h" => {
                print_usage(&args[0]);
                std::process::exit(0);
            }
            arg => {
                if input_path.is_none() {
                    input_path = Some(arg.to_string());
                } else if output_path.is_none() {
                    output_path = Some(arg.to_string());
                } else {
                    return Err(format!("Unexpected argument: {}", arg));
                }
                i += 1;
            }
        }
    }
    
    if input_path.is_none() || output_path.is_none() {
        return Err("Missing input or output file".to_string());
    }
    
    Ok(Config {
        input_path: input_path.unwrap(),
        output_path: output_path.unwrap(),
        parallel,
        threads,
    })
}

fn print_usage(program: &str) {
    eprintln!("usage: {} [OPTIONS] <input.hex|-) <output.bin|->", program);
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  -p, --parallel        並列処理を有効化（大幅な高速化）");
    eprintln!("  --threads N           使用するスレッド数を指定（デフォルト: CPU自動）");
    eprintln!("  -h, --help            このヘルプを表示");
    eprintln!();
    eprintln!("  Use '-' for stdin/stdout");
}

fn main() -> io::Result<()> {
    let config = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!();
        print_usage(&env::args().next().unwrap());
        std::process::exit(1);
    });

    // スレッド数の設定
    if let Some(num_threads) = config.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }

    // 入力ソースの選択
    let input_bytes: Vec<u8> = if config.input_path == "-" {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        buf
    } else {
        std::fs::read(&config.input_path)?
    };

    // 並列処理の場合はフィルタリングと変換を同時に並列化
    let bytes: Vec<u8> = if config.parallel {
        // 大きなチャンク単位で並列処理（フィルタリング + 変換を同時に）
        const CHUNK_SIZE: usize = 16 * 1024 * 1024; // 16MB単位
        
        input_bytes
            .par_chunks(CHUNK_SIZE)
            .flat_map(|chunk| {
                // 各チャンク内でフィルタリングと変換を実行
                let mut result = Vec::with_capacity(chunk.len() / 2);
                let mut hex_chars = Vec::with_capacity(chunk.len());
                
                // フィルタリング
                for &b in chunk {
                    if (b >= b'0' && b <= b'9') || (b >= b'A' && b <= b'F') || (b >= b'a' && b <= b'f') {
                        hex_chars.push(b);
                    }
                }
                
                // 16進数からバイナリへ変換
                for pair in hex_chars.chunks(2) {
                    if pair.len() == 2 {
                        result.push(hex_to_byte(pair[0], pair[1]));
                    }
                }
                
                result
            })
            .collect()
    } else {
        // シングルスレッド版
        let hexstr: Vec<u8> = input_bytes
            .iter()
            .copied()
            .filter(|&b| (b >= b'0' && b <= b'9') || (b >= b'A' && b <= b'F') || (b >= b'a' && b <= b'f'))
            .collect();
        
        hexstr
            .chunks(2)
            .map(|c| hex_to_byte(c[0], c[1]))
            .collect()
    };

    // 出力先の選択
    if config.output_path == "-" {
        let stdout = io::stdout();
        let mut handle = BufWriter::new(stdout.lock());
        handle.write_all(&bytes)?;
    } else {
        let mut output = BufWriter::new(File::create(&config.output_path)?);
        output.write_all(&bytes)?;
    }

    Ok(())
}
