use std::fmt;

use snafu::Backtrace;

#[inline]
#[must_use]
pub fn fmt_backtrace(backtrace: &Backtrace) -> String {
    if cfg!(feature = "backtrace") {
        format!("\n{}", backtrace)
    } else {
        "".to_string()
    }
}

#[inline]
#[must_use]
pub fn fmt_backtrace_with_source(backtrace: &Backtrace, source: impl fmt::Display) -> String {
    format!("{}{}", fmt_backtrace(backtrace), fmt_source(source))
}

#[inline]
#[must_use]
pub fn fmt_source(source: impl fmt::Display) -> String {
    format!("\nCaused by: {}", source)
}
