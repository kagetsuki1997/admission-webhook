use admission_webhook::{error, web};
use snafu::{Backtrace, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Web { source: web::error::Error },

    #[snafu(display(
        "Can not initialize Tokio runtime{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    InitializeTokioRuntime { source: tokio::io::Error, backtrace: Backtrace },

    #[snafu(display(
        "Can not initialize OpenTelemetry tracer{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    InitializeOpenTelemetryTracer { source: opentelemetry::trace::TraceError, backtrace: Backtrace },

    #[snafu(display(
        "Can not parse TLS config{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    ParseTlsConfigFromPemFile { source: std::io::Error, backtrace: Backtrace },
}

impl From<web::error::Error> for Error {
    fn from(error: web::error::Error) -> Self {
        Self::Web { source: error }
    }
}

// it occurs error[E0601]: `main` function not found in crate `error`
#[allow(dead_code)]
fn main() {}
