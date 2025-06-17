use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "calculate sm3",
    author = "DouShaoXun",
    // 编译时注入版本号
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Args {
    /// file path
    #[arg(short, long)]
    pub file: String,
}
