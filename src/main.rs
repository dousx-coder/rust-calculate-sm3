use clap::Parser;
use sm3::{Digest, Sm3};
use std::fs::File;
use std::fs::metadata;
use std::io::{Error, Read};
use std::path::Path;
#[derive(Parser, Debug)]
#[command(
    name = "calculate sm3",
    author = "DouShaoXun",
    // 编译时注入版本号
    version = env!("CARGO_PKG_VERSION"),
)]
struct Args {
    /// file path
    #[arg(short, long)]
    file: String,
}

fn check_file_exists<P>(path: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    match metadata(&path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn calculate_sm3_hash(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sm3::new();
    let mut bf = [0; 1024];
    loop {
        let bytes_read = file.read(&mut bf)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&bf[..bytes_read]);
    }
    let result = hasher.finalize();
    let sm3_hash = format!("{:x}", result);
    Ok(sm3_hash)
}
fn main() {
    let args = Args::parse();
    let file_path = &args.file;
    match check_file_exists(file_path) {
        Ok(exist) => {
            if exist {
                match calculate_sm3_hash(file_path) {
                    Ok(hash) => {
                        println!("SM3 Hash: {}", hash);
                    }
                    Err(e) => {
                        println!("Error: Failed to calculate SM3 hash.{file_path} {e}");
                        return;
                    }
                }
            } else {
                println!("file is not exists, {file_path}");
            }
        }
        Err(e) => {
            println!("Error: Failed to calculate SM3 hash.{file_path} {e}");
            return;
        }
    }
}
