mod arg;
mod calculate;

use crate::arg::args::Args;
use crate::calculate::sm3::calculate_sm3;
use clap::Parser;
use std::time::Instant;
fn main() {
    let instant = Instant::now();
    let args = Args::parse();
    let file = &args.file;
    match calculate_sm3(file) {
        Ok(sm3) => {
            let elapsed = instant.elapsed().as_millis();
            println!("Calculate SM3 Hash elapsed: {elapsed} ms");
            println!("SM3 Hash: {sm3}")
        }
        Err(e) => {
            eprintln!("Error:[{file}] calculate SM3 hash error! {e}");
            return;
        }
    }
}
