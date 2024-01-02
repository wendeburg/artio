use std::process;
use anyhow::Error;

pub fn handle_error_finish_execution<T>(error: Error) -> T {
    #[cfg(not(test))]
    {
        println!("Error: {}", error);
        process::exit(1);
    }

    #[cfg(test)]
    {
        panic!("handled error");
    }
}