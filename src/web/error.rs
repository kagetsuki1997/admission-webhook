use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use indexmap::IndexMap;
use snafu::Snafu;

use crate::web::{response, response::EncapsulatedJson};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Cannot mutate immutable field `{}`", field))]
    ImmutableField { field: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let err = match self {
            Self::ImmutableField { .. } => response::Error {
                type_: response::ErrorType::Internal,
                code: None,
                message: "Unexpected internal system error.".to_string(),
                additional_fields: IndexMap::default(),
            },
        };

        (EncapsulatedJson::<()>::err(err).status_code(self.status_code())).into_response()
    }
}

impl Error {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::ImmutableField { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
