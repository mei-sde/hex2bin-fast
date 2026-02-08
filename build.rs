fn main() {
    // Windowsの場合のみバージョン情報を埋め込む
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico")
            .set("ProductName", "hex2bin-fast")
            .set("FileDescription", "High-performance hexadecimal to binary converter")
            .set("CompanyName", "mei-sde")
            .set("LegalCopyright", "© 2026 mei-sde")
            .set("ProductVersion", "0.5.1");
        
        res.compile().unwrap();
    }
}
