use crate::messages::*;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    #[error("Unknown message type")]
    UnknownType,
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
    #[error("Rate limited")]
    RateLimited,
    #[error("Cooldown")]
    Cooldown,
    #[error("Not spawned")]
    NotSpawned,
    #[error("Invalid aim")]
    InvalidAim,
    #[error("Invalid slot")]
    InvalidSlot,
    #[error("Insufficient items")]
    InsufficientItems,
    #[error("Server busy")]
    ServerBusy,
}

impl ValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            ValidationError::InvalidMessage(_) => "invalid_message",
            ValidationError::UnknownType => "unknown_type",
            ValidationError::InvalidPayload(_) => "invalid_payload",
            ValidationError::RateLimited => "rate_limited",
            ValidationError::Cooldown => "cooldown",
            ValidationError::NotSpawned => "not_spawned",
            ValidationError::InvalidAim => "invalid_aim",
            ValidationError::InvalidSlot => "invalid_slot",
            ValidationError::InsufficientItems => "insufficient_items",
            ValidationError::ServerBusy => "server_busy",
        }
    }

    pub fn to_error_message(&self) -> ErrorData {
        ErrorData {
            code: self.code().to_string(),
            message: self.to_string(),
            details: None,
        }
    }
}

pub fn validate_input(data: &InputData, _max_range: f64) -> Result<(), ValidationError> {
    if !data.dx.is_finite() || !data.dy.is_finite() {
        return Err(ValidationError::InvalidPayload("Non-finite movement".into()));
    }

    if let Some(ref aim) = data.aim {
        if !aim.x.is_finite() || !aim.y.is_finite() {
            return Err(ValidationError::InvalidAim);
        }
    }

    if (data.attack || data.interact) && data.aim.is_none() {
        return Err(ValidationError::InvalidAim);
    }

    Ok(())
}

pub fn validate_slot(slot: usize, max_slots: usize) -> Result<(), ValidationError> {
    if slot >= max_slots {
        return Err(ValidationError::InvalidSlot);
    }
    Ok(())
}

pub fn validate_name(name: &str, max_len: usize) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::InvalidPayload("Name cannot be empty".into()));
    }
    if name.len() > max_len {
        return Err(ValidationError::InvalidPayload(format!(
            "Name exceeds max length of {}",
            max_len
        )));
    }
    Ok(())
}
