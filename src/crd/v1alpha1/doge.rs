use kube::{core::DynamicObject, CustomResource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::DogeStatus;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[kube(
    apiextensions = "v1",
    group = "app.demo",
    version = "v1alpha1",
    kind = "Doge",
    namespaced,
    struct = "Doge",
    derive = "Default",
    derive = "PartialEq"
)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub image: Option<String>,

    pub number: u64,

    pub doge_status: DogeStatus,
}

impl TryFrom<DynamicObject> for Doge {
    type Error = serde_json::error::Error;

    fn try_from(DynamicObject { metadata, data, .. }: DynamicObject) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata,
            spec: serde_json::from_value(
                data.as_object().expect("data should be Spec object.")["spec"].clone(),
            )?,
        })
    }
}
