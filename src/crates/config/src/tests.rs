#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_valid_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("server.json");
        let mut file = File::create(file_path).unwrap();
        write!(file, r#"{{
            "version": 1,
            "bind_http": "127.0.0.1:9090",
            "protocol_version": 3,
            "tick_rate": 20,
            "limits": {{
                "ws_max_message_bytes": 1024,
                "ws_messages_per_sec": 10,
                "ws_burst": 20,
                "max_name_len": 16
            }},
            "rate_limits": {{
                "session_claim_per_ip_per_minute": 5
            }}
        }}"#).unwrap();

        let config = crate::load_config(dir.path()).unwrap();
        assert_eq!(config.server.bind_http, "127.0.0.1:9090");
        assert_eq!(config.server.tick_rate, 20);
    }

    #[test]
    fn test_reject_unknown_fields() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("server.json");
        let mut file = File::create(file_path).unwrap();
        write!(file, r#"{{
            "version": 1,
            "bind_http": "0.0.0.0:8080",
            "protocol_version": 3,
            "tick_rate": 30,
            "limits": {{
                "ws_max_message_bytes": 16384,
                "ws_messages_per_sec": 30,
                "ws_burst": 60,
                "max_name_len": 24
            }},
            "rate_limits": {{
                "session_claim_per_ip_per_minute": 10
            }},
            "unknown_field": "should fail"
        }}"#).unwrap();

        let result = crate::load_config(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_defaults_on_missing_file() {
        let dir = tempdir().unwrap();
        // No files created
        let config = crate::load_config(dir.path()).unwrap();
        assert_eq!(config.server.tick_rate, 30); // Default
    }
}
