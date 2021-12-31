use serde::{Deserialize, Serialize};

/// Scan endpoint response
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub text: String,
}

/// Scan endpoint request
#[derive(Deserialize)]
pub struct Request {
    /// The min amount of confidence the image scan resulted in. Everything below will be treated
    /// as fail
    #[serde(default = "default_conf_threshold")]
    pub threshold: i32,
}

/// Default mit threshold value for detection confidence
#[inline]
fn default_conf_threshold() -> i32 {
    75
}
