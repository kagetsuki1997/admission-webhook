use snafu::{Backtrace, Snafu};

use crate::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "Invalid doge status: {value}, available values are [Normal, Crying, Raged, Buffed, \
         Parrot, Kachitoritai]{}",
        error::fmt_backtrace(backtrace)
    ))]
    InvalidDogeStatus { value: String, backtrace: Backtrace },
}
