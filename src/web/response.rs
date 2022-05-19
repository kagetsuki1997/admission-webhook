use axum::{
    body,
    http::{header, header::HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub struct EncapsulatedJson<T, M = ()> {
    status_code: StatusCode,

    metadata: Option<M>,

    result: Option<Payload<T>>,
}

impl<T> From<(StatusCode, T)> for EncapsulatedJson<T, ()> {
    #[inline]
    fn from((status_code, data): (StatusCode, T)) -> Self {
        Self { status_code, metadata: None, result: Some(Payload::ok(data)) }
    }
}

impl<T> From<(StatusCode, axum::Json<T>)> for EncapsulatedJson<T, ()> {
    #[inline]
    fn from((status_code, axum::Json(data)): (StatusCode, axum::Json<T>)) -> Self {
        Self { status_code, metadata: None, result: Some(Payload::ok(data)) }
    }
}

impl From<StatusCode> for EncapsulatedJson<(), ()> {
    #[inline]
    fn from(status_code: StatusCode) -> Self {
        Self { status_code, metadata: None, result: None }
    }
}

impl<T, M> From<Option<T>> for EncapsulatedJson<T, M> {
    #[inline]
    fn from(data: Option<T>) -> Self {
        Self { status_code: StatusCode::OK, metadata: None, result: data.map(Payload::ok) }
    }
}

impl<T, M> EncapsulatedJson<T, M> {
    #[inline]
    #[must_use]
    pub fn ok(data: T) -> Self {
        Self { status_code: StatusCode::OK, metadata: None, result: Some(Payload::ok(data)) }
    }

    #[inline]
    #[must_use]
    pub const fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl<M> EncapsulatedJson<(), M> {
    #[inline]
    #[must_use]
    pub fn err(error: Error) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            metadata: None,
            result: Some(Payload::err(error)),
        }
    }
}

impl<T, M> IntoResponse for EncapsulatedJson<T, M>
where
    T: Serialize,
    M: Serialize,
{
    fn into_response(self) -> Response {
        #[derive(Deserialize, Serialize)]
        struct JsonResponse<T, Metadata> {
            #[serde(rename = "_status")]
            status: u16,

            #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,

            #[serde(flatten, skip_serializing_if = "Option::is_none")]
            result: Option<Payload<T>>,
        }

        let Self { status_code, metadata, result } = self;

        let body = body::boxed(body::Full::from(
            serde_json::to_vec(&JsonResponse { status: status_code.as_u16(), metadata, result })
                .expect("json serialization never fail; qed"),
        ));

        let mut res = Response::new(body);
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        );
        *res.status_mut() = status_code;
        res
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Payload<R = ()> {
    #[serde(rename = "data")]
    Ok(Box<R>),

    #[serde(rename = "error")]
    Err(Box<Error>),
}

impl<R> Payload<R> {
    #[must_use]
    pub fn ok(data: R) -> Self {
        Self::Ok(Box::new(data))
    }

    #[must_use]
    pub fn err(error: Error) -> Self {
        Self::Err(Box::new(error))
    }
}

impl<R> From<Error> for Payload<R> {
    fn from(error: Error) -> Self {
        Self::err(error)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    Unknown,
    Internal,
    Validation,
    Execution,
    NotComplete,
    NotFound,
    Unauthorized,
    Request,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    #[serde(rename = "type")]
    pub type_: ErrorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    pub message: String,

    #[serde(flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub additional_fields: IndexMap<String, serde_json::Value>,
}
