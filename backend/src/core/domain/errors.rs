use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum DomainError {
    InvalidStateTransition {
        from: String,
        to: String,
        entity: String,
    },
    BusinessRuleViolation(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidStateTransition { from, to, entity } => {
                write!(f, "Invalid state transition for {}: {} -> {}", entity, from, to)
            }
            DomainError::BusinessRuleViolation(msg) => write!(f, "Business rule violation: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: Option<String>,
    pub correlation_id: Option<String>,
}

impl ApiErrorResponse {
    pub fn new(message: impl Into<String>, code: Option<String>, correlation_id: Option<String>) -> Self {
        Self {
            message: message.into(),
            code,
            correlation_id,
        }
    }
}
