mod config;

use std::{fmt, io::Write, process};

use admission_webhook::{
    domain::{DogeConfig, MutatingConfig},
    web,
};
use axum::http::Uri;
use axum_server::tls_rustls::RustlsConfig;
use bb8::ErrorSink;
use clap::{IntoApp, Parser, Subcommand};
use clap_complete::Shell;
use lifecycle_manager::LifecycleManager;
use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_NAMESPACE};
use snafu::ResultExt;
use tokio::runtime::Runtime;
use tracing::Instrument;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{error, error::Result};

use self::config::Config;

const APP_NAME: &str = "Admission Webhook";

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

impl Default for Cli {
    #[inline]
    fn default() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Shows current version")]
    Version,

    #[clap(about = "Shows shell completions")]
    Completions { shell: Shell },

    #[clap(name = "run", aliases = &["execution"], about = "Run Execution")]
    Run {
        #[clap(flatten)]
        config: Box<Config>,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.commands {
            Commands::Version => {
                let mut stdout = std::io::stdout();
                stdout
                    .write_all(Self::command().render_long_version().as_bytes())
                    .expect("failed to write to stdout");
                Ok(())
            }
            Commands::Completions { shell } => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                Ok(())
            }
            Commands::Run { config } => {
                let Config { api, tls, doge, telemetry } = *config;

                Runtime::new().context(error::InitializeTokioRuntimeSnafu)?.block_on(
                    async move {
                        init_tracing(telemetry.endpoint())?;
                        tracing::debug!("{APP_NAME} starting");

                        tracing::info!("Process ID: {}", process::id());

                        let lifecycle = async {
                            let mut lifecycle = LifecycleManager::<error::Error>::new();

                            let tls_config = RustlsConfig::from_pem_file(tls.cert, tls.key)
                                .await
                                .context(error::ParseTlsConfigFromPemFileSnafu)?;

                            let mutating_config = {
                                let doge_config = DogeConfig {
                                    default_image: doge.default_image,
                                    default_number: doge.default_number,
                                    default_status: doge.default_status,
                                };
                                MutatingConfig { default_doge: doge_config }
                            };

                            tracing::info!("Initializing API server");
                            lifecycle = lifecycle.add_worker(web::new_api_server_with_tls(
                                api.socket_address(),
                                mutating_config,
                                tls_config,
                            )?);

                            Result::Ok(lifecycle)
                        }
                        .instrument(tracing::info_span!("initializing"))
                        .await?;

                        tracing::info!("{APP_NAME} started");
                        lifecycle.serve().await
                    },
                )?;

                tracing::info!("{APP_NAME} shutdown complete");

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LoggingErrorSink;

impl<E> ErrorSink<E> for LoggingErrorSink
where
    E: fmt::Display,
{
    fn sink(&self, err: E) {
        tracing::warn!("error in pool: {err}")
    }

    fn boxed_clone(&self) -> Box<dyn ErrorSink<E>> {
        Box::new(*self)
    }
}

fn init_tracing(endpoint: Option<&Uri>) -> Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    // filter
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=warn"));
    // telemetry
    let telemetry_layer = if let Some(endpoint) = endpoint {
        let exporter =
            opentelemetry_otlp::new_exporter().tonic().with_endpoint(endpoint.to_string());

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(trace::config().with_resource(Resource::new(vec![
                KeyValue::new(SERVICE_NAME, APP_NAME),
                KeyValue::new(SERVICE_NAMESPACE, "Saffron"),
            ])))
            .install_batch(runtime::Tokio)
            .context(error::InitializeOpenTelemetryTracerSnafu)?;

        Some(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        None
    };
    // format
    let fmt_layer =
        tracing_subscriber::fmt::layer().pretty().with_thread_ids(true).with_thread_names(true);
    // subscriber
    tracing_subscriber::registry().with(filter_layer).with(telemetry_layer).with(fmt_layer).init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{Cli, Commands};

    #[test]
    fn test_command_simple() {
        match Cli::parse_from(&["program_name", "version"]).commands {
            Commands::Version => (),
            _ => panic!(),
        }
    }
}
