use std::process;
use anyhow::Error;

pub fn handle_error_finish_execution<T>(error: Error) -> T {
    println!("Error: {}", error);
    process::exit(1);
}