use std::{fs::File, io::Read};

use anyhow::Result;
use customs_rs::process::process;

fn main() -> Result<()> {
    let filename = "customs.i4";
    println!("\nFilename: {}\n", &filename);
    let mut file = File::open(format!("data/input/{}", &filename))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result = process(&contents);
    print!("{result}");
    Ok(())
}
