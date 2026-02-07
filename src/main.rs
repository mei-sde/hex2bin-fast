use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <input.hex|-) <output.bin|->", args[0]);
        eprintln!("  Use '-' for stdin/stdout");
        std::process::exit(1);
    }

    // 入力ソースの選択
    let mut input = String::new();
    if args[1] == "-" {
        io::stdin().read_to_string(&mut input)?;
    } else {
        File::open(&args[1])?.read_to_string(&mut input)?;
    }

    // 有効な16進数文字のみを抽出
    let hexstr: String = input
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();

    let bytes: Vec<u8> = hexstr
        .as_bytes()
        .chunks(2)
        .map(|c| {
            let hi = (c[0] as char).to_digit(16).unwrap();
            let lo = (c[1] as char).to_digit(16).unwrap();
            (hi * 16 + lo) as u8
        })
        .collect();

    // 出力先の選択
    if args[2] == "-" {
        let stdout = io::stdout();
        let mut handle = BufWriter::new(stdout.lock());
        handle.write_all(&bytes)?;
    } else {
        let mut output = File::create(&args[2])?;
        output.write_all(&bytes)?;
    }

    Ok(())
}
