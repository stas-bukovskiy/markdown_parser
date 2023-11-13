extern crate clap;

use clap::{Parser, Subcommand};
use markdown_parser::parse_markdown;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(name = "markdown-parser-cli")]
#[command(author = "Stanislav Bukovskyi <stas.bukovskyi@@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Parse markdown to HTML", long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value = "output.html")]
    out: String,
    #[arg(short, long, default_value = "false")]
    is_force: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Credits {},
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading or writing file `{0}`")]
    IOError(#[from] std::io::Error),
    #[error("Error processing command line arguments: {0}")]
    ClapError(String),
}

fn main() -> Result<(), AppError> {
    let cli = Args::try_parse();
    match &cli {
        Ok(cli) => match &cli.command {
            Some(Commands::Credits {}) => {
                println!("Credits: ");
                println!("Stanislav Bukovskyi <stas.bukovskyi@@gmail.com>");
            }
            None => {
                let input_file_result = read_from_file(cli.file.clone());
                match input_file_result {
                    Ok(input_file) => {
                        let output_content = parse_markdown(&input_file.clone());

                        match write_to_file(&output_content, &cli.out, cli.is_force) {
                            Ok(_) => println!("File written successfully."),
                            Err(err) => {
                                return Err(AppError::IOError(err));
                            }
                        }
                    }
                    Err(err) => {
                        return Err(AppError::IOError(err));
                    }
                };
            }
        },
        Err(err) => {
            return Err(AppError::ClapError(err.to_string()));
        }
    }

    Ok(())
}

fn read_from_file(file_path: String) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_to_file(content: &str, file_name: &str, is_force: bool) -> io::Result<()> {
    let file_path = file_name;
    let file_exists = std::path::Path::new(&file_path).exists();

    if file_exists && !is_force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists and is_force flag is not set",
        ));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(is_force)
        .open(file_path)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}
