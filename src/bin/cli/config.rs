use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::http::Uri;
use clap::Args;

use admission_webhook::{domain::DogeStatus, env};

#[derive(Args, Debug)]
pub struct Config {
    #[clap(flatten)]
    pub api: ApiConfig,

    #[clap(flatten)]
    pub tls: TlsConfig,

    #[clap(flatten)]
    pub doge: DogeConfig,

    #[clap(flatten)]
    pub telemetry: TelemetryConfig,
}

#[derive(Args, Debug)]
pub struct ApiConfig {
    #[clap(
        name = "api-address",
        long,
        env = env::API_ADDRESS,
        default_value = "127.0.0.1"
    )]
    pub address: IpAddr,

    #[clap(
        name = "api-port",
        long,
        env = env::API_PORT,
        default_value = "8007"
    )]
    pub port: u16,
}

impl ApiConfig {
    #[inline]
    pub fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }
}

#[derive(Args, Debug)]
pub struct TlsConfig {
    #[clap(
        name = "tls-cert",
        long,
        env = env::TLS_CERT,
    )]
    pub cert: PathBuf,

    #[clap(
        name = "tls-key",
        long,
        env = env::TLS_KEY,
    )]
    pub key: PathBuf,
}

#[derive(Args, Debug)]
pub struct DogeConfig {
    #[clap(
        name = "doge-default-image",
        long,
        env = env::DOGE_DEFAULT_IMAGE,
        default_value="doge/doge:doge"
    )]
    pub default_image: String,

    #[clap(
        name = "doge-default-number",
        long,
        env = env::DOGE_DEFAULT_NUMBER,
        default_value="87"
    )]
    pub default_number: u64,

    #[clap(
        name = "doge-default-status",
        long,
        env = env::DOGE_DEFAULT_STATUS,
        default_value="Normal"
    )]
    pub default_status: DogeStatus,
}

#[derive(Args, Debug)]
pub struct TelemetryConfig {
    #[clap(
        name = "telemetry-otlp-endpoint",
        long,
        env = env::TELEMETRY_OTLP_ENDPOINT,
    )]
    endpoint: Option<Uri>,
}

impl TelemetryConfig {
    #[inline]
    pub fn endpoint(&self) -> Option<&Uri> {
        self.endpoint.as_ref()
    }
}
