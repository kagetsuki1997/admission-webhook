mod error;
mod model;

pub use model::{DogeConfig, DogeStatus};

#[derive(Clone)]
pub struct MutatingConfig {
    pub default_doge: DogeConfig,
}
