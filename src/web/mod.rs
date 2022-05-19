mod controller;
pub mod error;
mod response;

use std::{fmt, marker::PhantomData, net::SocketAddr, time::Duration};

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::Extension,
    handler::Handler,
    http::{StatusCode, Uri},
    Router,
};
use axum_server::{
    tls_rustls::{RustlsAcceptor, RustlsConfig},
    Handle,
};
use lifecycle_manager::{ShutdownSignal, Worker};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::{domain::MutatingConfig, web, web::response::EncapsulatedJson};

/// # Errors
///
/// * if it cannot create runtime
/// * if it cannot bind server
pub fn new_api_server_with_tls<E>(
    socket_address: SocketAddr,
    config: MutatingConfig,
    tls_config: RustlsConfig,
) -> Result<AxumWebServer<E>, E>
where
    E: std::error::Error + std::convert::From<web::error::Error> + Send,
{
    let middleware_stack =
        ServiceBuilder::new().layer(TraceLayer::new_for_http()).layer(CompressionLayer::new());

    let router = self::controller::api_v1_index().layer(Extension(config)).layer(middleware_stack);

    Ok(AxumWebServer::new("API Server", socket_address, router, tls_config))
}

pub struct AxumWebServer<E> {
    name: String,
    socket_address: SocketAddr,
    router: Router<Body>,
    error_type: PhantomData<E>,
    tls_config: RustlsConfig,
}

impl<E> AxumWebServer<E>
where
    E: From<error::Error> + Send,
{
    pub fn new<T>(
        name: T,
        socket_address: SocketAddr,
        router: Router<Body>,
        tls_config: RustlsConfig,
    ) -> Self
    where
        T: fmt::Display,
    {
        Self {
            name: name.to_string(),
            socket_address,
            router,
            error_type: PhantomData::default(),
            tls_config,
        }
    }
}

#[async_trait]
impl<E> Worker for AxumWebServer<E>
where
    E: From<error::Error> + Send,
{
    type Error = E;

    fn name(&self) -> &str {
        &self.name
    }

    async fn serve(mut self, shutdown_signal: ShutdownSignal) -> Result<(), Self::Error> {
        let handle = Handle::new();
        tokio::spawn({
            let handle = handle.clone();

            async move {
                tracing::debug!("Start to listen graceful shutdown signal");

                shutdown_signal.await;

                handle.graceful_shutdown(Some(Duration::from_secs(10)));
            }
        });

        let router = {
            self.router
                .fallback(fallback.into_service())
                .into_make_service_with_connect_info::<SocketAddr>()
        };

        tracing::debug!("HTTP TLS server started on {}", self.socket_address);

        let server = axum_server::bind(self.socket_address)
            .acceptor(RustlsAcceptor::new(self.tls_config))
            .handle(handle)
            .serve(router);

        if let Err(err) = server.await {
            tracing::warn!("Error occurs while awaiting for AxumWebTlsServer {err}");
        }

        tracing::info!("AxumWebTlsServer is gracefully shutdown");

        Ok(())
    }
}

#[allow(clippy::unused_async)]
async fn fallback(uri: Uri) -> EncapsulatedJson<()> {
    EncapsulatedJson::<()>::err(response::Error {
        type_: response::ErrorType::NotFound,
        code: None,
        message: format!("No route for {}", uri),
        additional_fields: indexmap::IndexMap::default(),
    })
    .status_code(StatusCode::NOT_FOUND)
}
