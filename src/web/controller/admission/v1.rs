use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
    Json,
};
use json_patch::{AddOperation, Patch, PatchOperation, ReplaceOperation};
use kube::core::{
    admission::{AdmissionRequest, AdmissionResponse, AdmissionReview, Operation},
    DynamicObject, ResourceExt,
};

use crate::{
    crd::v1alpha1::Doge,
    domain::MutatingConfig,
    web::{error, response::EncapsulatedJson},
};

// Referenced from https://github.com/kube-rs/kube-rs/blob/master/examples/admission_controller.rs
#[allow(clippy::unused_async)]
pub async fn mutate_handler(
    Extension(config): Extension<MutatingConfig>,
    Json(body): Json<AdmissionReview<DynamicObject>>,
) -> error::Result<Response> {
    tracing::info!("Mutating Request: {:?}", body);

    // 1. Parse incoming webhook AdmissionRequest first
    let request: AdmissionRequest<_> = match body.try_into() {
        Ok(request) => request,
        Err(err) => {
            tracing::error!("invalid request: {}", err.to_string());
            return Ok(EncapsulatedJson::<_, ()>::ok(
                AdmissionResponse::invalid(err.to_string()).into_review(),
            )
            .into_response());
        }
    };

    // 2. Then construct a AdmissionResponse
    let mut res = AdmissionResponse::from(&request);
    // request.Object always exists for us, but could be None if extending to DELETE
    // events
    if let Some(obj) = request.object {
        // 3. mutate
        res = match mutate(res.clone(), &obj, &config) {
            Ok(res) => {
                tracing::info!("accepted: {:?} {}", request.operation, obj.name());
                res
            }
            Err(err) => {
                tracing::warn!("denied: {:?} {} ({})", request.operation, obj.name(), err);
                res.deny(err.to_string())
            }
        };
    };

    // 4. Wrap the AdmissionResponse wrapped in an AdmissionReview
    tracing::info!("Mutating Response: {:?}", res.clone().into_review());
    Ok(Json(res.into_review()).into_response())
}

// The main handler and core business logic, failures here implies rejected
// applies
fn mutate(
    res: AdmissionResponse,
    obj: &DynamicObject,
    config: &MutatingConfig,
) -> Result<AdmissionResponse, Box<dyn std::error::Error>> {
    if let Some(type_meta) = obj.types.as_ref() {
        let patches = match type_meta.kind.as_str() {
            "Doge" => mutate_doge(&Doge::try_from(obj.clone())?, config),
            _ => return Ok(res),
        };

        Ok(res.with_patch(Patch(patches))?)
    } else {
        Ok(res)
    }
}

fn mutate_doge(doge: &Doge, config: &MutatingConfig) -> Vec<PatchOperation> {
    let mut patches = Vec::<PatchOperation>::default();

    if doge.spec.image.is_none() {
        patches.push(PatchOperation::Add(AddOperation {
            path: "/spec/image".into(),
            value: serde_json::Value::String(config.default_doge.default_image.clone()),
        }));
    }

    patches.push(PatchOperation::Replace(ReplaceOperation {
        path: "/spec/number".into(),
        value: serde_json::Value::Number(config.default_doge.default_number.into()),
    }));

    patches
}

pub async fn validate_handler(
    Json(body): Json<AdmissionReview<DynamicObject>>,
) -> error::Result<Response> {
    tracing::info!("Validating Request: {:?}", body);
    // 1. Parse incoming webhook AdmissionRequest first
    let request: AdmissionRequest<_> = match body.try_into() {
        Ok(request) => request,
        Err(err) => {
            tracing::error!("invalid request: {}", err.to_string());
            return Ok(EncapsulatedJson::<_, ()>::ok(
                AdmissionResponse::invalid(err.to_string()).into_review(),
            )
            .into_response());
        }
    };

    // 2. Then construct a AdmissionResponse
    let mut res = AdmissionResponse::from(&request);
    // request.Object always exists for us, but could be None if extending to DELETE
    // events
    if let Some(obj) = request.object {
        // 3. validate
        res = match validate(&request.operation, res.clone(), &obj, &request.old_object) {
            Ok(res) => {
                tracing::info!("accepted: {:?} {}", request.operation, obj.name());
                res
            }
            Err(err) => {
                tracing::warn!("denied: {:?} {} ({})", request.operation, obj.name(), err);
                res.deny(err.to_string())
            }
        };
    };

    // 4. Wrap the AdmissionResponse wrapped in an AdmissionReview
    tracing::info!("Validating Response: {:?}", res.clone().into_review());
    Ok(Json(res.into_review()).into_response())
}

// The main handler and core business logic, failures here implies rejected
// applies
fn validate(
    operation: &Operation,
    res: AdmissionResponse,
    obj: &DynamicObject,
    maybe_old_obj: &Option<DynamicObject>,
) -> Result<AdmissionResponse, Box<dyn std::error::Error>> {
    if let Some(type_meta) = obj.types.as_ref() {
        match type_meta.kind.as_str() {
            "Doge" => validate_doge(operation, &Doge::try_from(obj.clone())?, maybe_old_obj),
            _ => Ok(()),
        }
        .map(|()| res)
    } else {
        Ok(res)
    }
}

fn validate_doge(
    operation: &Operation,
    doge: &Doge,
    maybe_old_obj: &Option<DynamicObject>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. General validation

    // 2. Validate for specific operation
    if *operation == Operation::Update {
        if let Some(old_obj) = maybe_old_obj {
            let old_doge = Doge::try_from(old_obj.clone())?;

            if doge.metadata.name.ne(&old_doge.metadata.name) {
                return Err(Box::new(error::Error::ImmutableField {
                    field: "metadata.name".to_string(),
                }));
            }

            if doge.spec.image.ne(&old_doge.spec.image) {
                return Err(Box::new(error::Error::ImmutableField {
                    field: "spec.image".to_string(),
                }));
            }
        }
    }

    Ok(())
}
