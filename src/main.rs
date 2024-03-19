#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::{fs::File, io::Read};

use anyhow::Result;
use customs_rs::process::process;

fn main() -> Result<()> {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let filename = "customs.i9";
    println!("\nFilename: {}\n", &filename);
    let mut file = File::open(format!("data/input/{}", &filename))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    process(&contents);
    Ok(())
}
