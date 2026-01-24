#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_encode_decode() {
        let msg = Message {
            protocol_version: 1,
            msg_type: MessageType::Input(InputData {
                player_id: 123,
                action: "move".to_string(),
                data: vec![1, 2, 3],
            }),
            seq: 42,
            payload: vec![],
        };
        let encoded = msg.encode().unwrap();
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}