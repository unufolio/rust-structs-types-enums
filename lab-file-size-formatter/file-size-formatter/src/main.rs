use std::env;

struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl Sizes {
    fn from_file_size(file_size: FileSize) -> Self {
        match file_size {
            FileSize::Bytes(b) => Self::from_bytes(b),
            FileSize::Kilobytes(kb) => Self::from_kilobytes(kb),
            FileSize::Megabytes(mb) => Self::from_megabytes(mb),
            FileSize::Gigabytes(gb) => Self::from_gigabytes(gb),
        }
    }

    fn from_bytes(bytes: u64) -> Self {
        Self {
            bytes,
            kilobytes: bytes / 1_000,
            megabytes: bytes / 1_000_000,
            gigabytes: bytes / 1_000_000_000,
        }
    }

    fn from_kilobytes(kilobytes: u64) -> Self {
        let bytes = kilobytes * 1_000;
        Self {
            bytes,
            kilobytes,
            megabytes: kilobytes / 1_000,
            gigabytes: kilobytes / 1_000_000,
        }
    }

    fn from_megabytes(megabytes: u64) -> Self {
        let kilobytes = megabytes * 1_000;
        let bytes = kilobytes * 1_000;
        Self {
            bytes,
            kilobytes,
            megabytes,
            gigabytes: megabytes / 1_000,
        }
    }

    fn from_gigabytes(gigabytes: u64) -> Self {
        let megabytes = gigabytes * 1_000;
        let kilobytes = megabytes * 1_000;
        let bytes = kilobytes * 1_000;
        Self {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 如果参数参数长度小于 2
    if args.len() < 2 {
        eprintln!("Usage: file-size-formatter <filesize>");
        return;
    }

    let input_str = &args[1];

    match parse_filesize(input_str) {
        Ok(file_size) => {
            let sizes = Sizes::from_file_size(file_size);
            println!(
                "Sizes {{ bytes: {} bytes, kilobytes: {} kilobytes,\
                 megabytes: {} megabytes, gigabytes: {} gigabytes }}",
                sizes.bytes, sizes.kilobytes, sizes.megabytes, sizes.gigabytes
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn parse_filesize(input: &str) -> Result<(FileSize), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Input should be in the format '<value> <unit>'".to_string());
    }
    let value: u64 = parts[0]
        .parse()
        .map_err(|_| "Invalid number format. Provide a valid number.".to_string())?;

    let unit = parts[1].to_lowercase();

    match unit.as_str() {
        "b" => Ok(FileSize::Bytes(value)),
        "kb" => Ok(FileSize::Kilobytes(value)),
        "mb" => Ok(FileSize::Megabytes(value)),
        "gb" => Ok(FileSize::Gigabytes(value)),
        _ => Err("Invalid unit. Supported units: b, kb, mb, gb.".to_string()),
    }
}
