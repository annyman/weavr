use note_compiler::*;
use std::path::Path;
use anyhow::{Result, anyhow};

fn main() -> Result<(), anyhow::Error> {
    let input_file = "example.note";
    let json = process_note_file(input_file)?;
    
    let output_file = Path::new(input_file).with_extension("json");
    write_json_to_file(&json, output_file.to_str()
        .ok_or_else(|| anyhow!("Invalid output file path"))?)?;
    
    println!("Processed {} -> {}", input_file, output_file.display());
    Ok(())
}