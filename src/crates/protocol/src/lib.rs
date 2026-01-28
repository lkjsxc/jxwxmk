mod models;
pub use models::*;

pub fn init() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_input_deserialization() {
        let json = r#"{"type": "input", "data": { "dx": -1.0, "dy": 0.0, "attack": false, "interact": false, "aim": { "x": 12.5, "y": 9.0 } } }"#;
        let msg: ClientMessage = from_str(json).unwrap();
        match msg {
            ClientMessage::Input(data) => {
                assert_eq!(data.dx, -1.0);
                assert_eq!(data.aim.unwrap().x, 12.5);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_input_missing_aim_is_ok_if_not_validated_here() {
        // Serialization allows option, validation logic (game crate) checks if it's required.
        let json = r#"{"type": "input", "data": { "dx": 0.0, "dy": 0.0, "attack": false, "interact": false, "aim": null } }"#;
        let msg: ClientMessage = from_str(json).unwrap();
         match msg {
            ClientMessage::Input(data) => {
                assert!(data.aim.is_none());
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_unknown_field_rejection() {
        let json = r#"{"type": "craft", "data": { "recipe": "pickaxe", "extra": "bad" } }"#;
        let res: Result<ClientMessage, _> = from_str(json);
        assert!(res.is_err());
    }
}