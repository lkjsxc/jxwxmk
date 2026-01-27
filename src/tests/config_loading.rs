use std::fs;

use jxwxmk::config::Config;

#[test]
fn config_loads_defaults_and_overrides() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("server.json"),
        r#"{ "tick_rate": 30.0, "http_port": 9090 }"#,
    )
    .unwrap();
    let config = Config::load_from_dir(dir.path()).unwrap();
    assert_eq!(config.server.tick_rate, 30.0);
    assert_eq!(config.server.http_port, 9090);
    assert_eq!(config.world.chunk_size, Config::default().world.chunk_size);
}
