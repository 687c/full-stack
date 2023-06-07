use derive_more::Display;
use serde::{Deserialize, Serialize};

// Helper error conversion types

#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
#[display(fmt = "ApiError: {}", detail)]
pub struct ApiError {
    pub status_code: u16,
    pub detail: String,
}

impl ApiError {
    pub fn new_internal(detail: String) -> ApiError {
        error!("Internal server error: {}", &detail);
        ApiError {
            status_code: 500,
            detail: format!("Internal Server Error: {}", detail),
        }
    }
}
