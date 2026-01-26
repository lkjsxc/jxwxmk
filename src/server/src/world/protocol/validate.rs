use std::fmt;

use super::{ClientInput, ClientMessage, MovementInput, PROTOCOL_VERSION};

#[derive(Debug, Default, Clone)]
pub struct SessionSequence {
    last_seq: u64,
}

impl SessionSequence {
    pub fn last_seq(&self) -> u64 {
        self.last_seq
    }
}

#[derive(Debug, Clone)]
pub struct ValidatedInput {
    pub seq: u64,
    pub input: ClientInput,
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    ProtocolVersionMismatch,
    SequenceOutOfOrder { last_seq: u64, received_seq: u64 },
    MovementOutOfRange { dx: i8, dy: i8 },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::ProtocolVersionMismatch => {
                write!(f, "protocol version mismatch")
            }
            ValidationError::SequenceOutOfOrder {
                last_seq,
                received_seq,
            } => write!(
                f,
                "sequence out of order: last={}, received={}",
                last_seq, received_seq
            ),
            ValidationError::MovementOutOfRange { dx, dy } => {
                write!(f, "movement out of range: dx={}, dy={}", dx, dy)
            }
        }
    }
}

pub fn validate_input_message(
    message: ClientMessage,
    seq: &mut SessionSequence,
) -> Result<ValidatedInput, ValidationError> {
    match message {
        ClientMessage::Input {
            protocol_version,
            seq: message_seq,
            input,
        } => {
            if protocol_version != PROTOCOL_VERSION {
                return Err(ValidationError::ProtocolVersionMismatch);
            }
            if message_seq <= seq.last_seq {
                return Err(ValidationError::SequenceOutOfOrder {
                    last_seq: seq.last_seq,
                    received_seq: message_seq,
                });
            }

            validate_movement(&input.movement)?;

            seq.last_seq = message_seq;

            Ok(ValidatedInput {
                seq: message_seq,
                input,
            })
        }
    }
}

fn validate_movement(movement: &MovementInput) -> Result<(), ValidationError> {
    let dx = movement.dx;
    let dy = movement.dy;
    if dx < -1 || dx > 1 || dy < -1 || dy > 1 {
        return Err(ValidationError::MovementOutOfRange { dx, dy });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_wrong_protocol_version() {
        let mut seq = SessionSequence::default();
        let message = ClientMessage::Input {
            protocol_version: PROTOCOL_VERSION + 1,
            seq: 1,
            input: ClientInput {
                movement: MovementInput { dx: 0, dy: 0 },
            },
        };

        let result = validate_input_message(message, &mut seq);
        assert!(matches!(result, Err(ValidationError::ProtocolVersionMismatch)));
    }

    #[test]
    fn enforces_monotonic_sequences() {
        let mut seq = SessionSequence::default();
        let first = ClientMessage::Input {
            protocol_version: PROTOCOL_VERSION,
            seq: 2,
            input: ClientInput {
                movement: MovementInput { dx: 0, dy: 0 },
            },
        };
        let second = ClientMessage::Input {
            protocol_version: PROTOCOL_VERSION,
            seq: 1,
            input: ClientInput {
                movement: MovementInput { dx: 0, dy: 0 },
            },
        };

        assert!(validate_input_message(first, &mut seq).is_ok());
        let result = validate_input_message(second, &mut seq);
        assert!(matches!(result, Err(ValidationError::SequenceOutOfOrder { .. })));
    }

    #[test]
    fn rejects_out_of_range_movement() {
        let mut seq = SessionSequence::default();
        let message = ClientMessage::Input {
            protocol_version: PROTOCOL_VERSION,
            seq: 1,
            input: ClientInput {
                movement: MovementInput { dx: 3, dy: 0 },
            },
        };

        let result = validate_input_message(message, &mut seq);
        assert!(matches!(result, Err(ValidationError::MovementOutOfRange { .. })));
    }
}
