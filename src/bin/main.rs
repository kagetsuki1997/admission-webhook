mod cli;
mod error;

use std::{fs, process};

use self::cli::Cli;

#[cfg(not(miri))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
const TERMINATION_MESSAGE_PATH: &str = "/dev/termination-log";

fn main() {
    if let Err(err) = Cli::default().run() {
        write_termination_message(err.to_string());
        eprintln!("{}", err);
        process::exit(-1);
    }
}

#[inline]
pub fn write_termination_message(message: impl AsRef<[u8]>) {
    if matches! (fs::metadata(TERMINATION_MESSAGE_PATH), Ok(metadata) if metadata.is_file()) {
        if let Err(err) = fs::write(TERMINATION_MESSAGE_PATH, message) {
            eprintln!("fail to write termination message: {}", err);
        }
    }
}
