use std::fs;
use std::path::Path;
use super::*;
use log::info;

pub fn load_config(config_dir: &Path) -> GameConfig {
    info!("Loading config from {:?}", config_dir);
    
    let server_path = config_dir.join("server.json");
    let world_path = config_dir.join("world.json");

    let server: ServerConfig = serde_json::from_str(&fs::read_to_string(&server_path).expect("Failed to read server.json")).expect("Failed to parse server.json");
    let world: WorldConfig = serde_json::from_str(&fs::read_to_string(&world_path).expect("Failed to read world.json")).expect("Failed to parse world.json");

    GameConfig { server, world }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path();

        let server_json = r#"{ "port": 9090, "tick_rate": 30 }"#;
        let world_json = r#"{ "seed": 999, "chunk_size": 32 }"#;

        File::create(config_dir.join("server.json")).unwrap().write_all(server_json.as_bytes()).unwrap();
        File::create(config_dir.join("world.json")).unwrap().write_all(world_json.as_bytes()).unwrap();

        let config = load_config(config_dir);

        assert_eq!(config.server.port, 9090);
        assert_eq!(config.world.seed, 999);
    }
}
