use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let output = convat_yaml2json::convert(&input)?;
    print!("{output}");
    Ok(())
}
