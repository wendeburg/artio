use std::process;
use anyhow::Error;

pub fn handle_error_finish_execution<T>(error: Error) -> T {
    #[cfg(not(test))]
    {
        eprintln!("Error: {}", error);
        process::exit(1);
    }

    #[cfg(test)]
    {
        panic!("handled error");
    }
}

pub fn handle_warning_print_to_stdout(error: Error) {
    println!("Warning: {}", error);
}