use clap::Parser;
use note_compiler::*;
use std::path::Path;
use std::io::{Write, stderr};
use anyhow::{Result, Context, anyhow};

#[derive(Parser)]
struct Cli {
    /// Path to the input file
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let cli = Cli::parse();
    let input_file = cli.input_file;

    match process_and_write_file(&input_file) {
        Ok(_) => println!("File processed successfully."),
        Err(e) => handle_error(&e, &input_file),
    }
}

fn process_and_write_file(input_file: &str) -> Result<(), anyhow::Error> {
    let json = process_note_file(input_file)
        .with_context(|| format!("Failed to process file '{}'. Ensure the file exists and is readable.", input_file))?;
    
    let output_path = Path::new(input_file).with_extension("json");
    let output_path_str = output_path.to_str()
        .ok_or_else(|| anyhow!("Invalid output file path"))?;
    
    write_json_to_file(&json, output_path_str)
        .with_context(|| format!("Failed to write JSON to '{}'. Check permissions or disk space.", output_path.display()))?;
    
    println!("Processed {} -> {}", input_file, output_path.display());
    Ok(())
}

fn handle_error(error: &anyhow::Error, input_file: &str) {
    writeln!(stderr(), "Error: {}", error).unwrap();
    
    if let Some(io_error) = error.downcast_ref::<std::io::Error>() {
        match io_error.kind() {
            std::io::ErrorKind::NotFound => {
                writeln!(stderr(), "Suggestion: Check if the file '{}' exists in the current directory.", input_file).unwrap();
            },
            std::io::ErrorKind::PermissionDenied => {
                writeln!(stderr(), "Suggestion: Ensure you have read/write permissions for the specified file.").unwrap();
            },
            _ => {
                writeln!(stderr(), "Suggestion: An unexpected I/O error occurred. Please check the file path and try again.").unwrap();
            }
        }
    } else {
        writeln!(stderr(), "Suggestion: Inspect the error message above for more details.").unwrap();
    }
}
