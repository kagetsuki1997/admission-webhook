use std::{fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::error;

#[derive(Clone)]
pub struct DogeConfig {
    pub default_image: String,
    pub default_number: u64,
    pub default_status: DogeStatus,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "PascalCase")]
pub enum DogeStatus {
    Normal,
    Crying,
    Raged,
    Buffed,
    Parrot,
    Kachitoritai,
}

impl Default for DogeStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for DogeStatus {
    type Err = error::Error;

    fn from_str(s: &str) -> error::Result<Self> {
        match s.to_lowercase().as_ref() {
            "normal" => Ok(Self::Normal),
            "crying" => Ok(Self::Crying),
            "raged" => Ok(Self::Raged),
            "buffed" => Ok(Self::Buffed),
            "parrot" => Ok(Self::Parrot),
            "kachitoritai" => Ok(Self::Kachitoritai),
            _ => Err(error::InvalidDogeStatusSnafu { value: s.to_string() }.build()),
        }
    }
}

impl DogeStatus {
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Normal => "Normal",
            Self::Crying => "Crying",
            Self::Raged => "Raged",
            Self::Buffed => "Buffed",
            Self::Parrot => "Parrot",
            Self::Kachitoritai => "Kachitoritai",
        }
    }
}

impl fmt::Display for DogeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
