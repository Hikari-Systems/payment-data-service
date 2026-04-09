use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub db: hs_utils::db::DbConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    #[serde(deserialize_with = "hs_utils::config::deser_u16_or_str")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub level: String,
}

pub fn load() -> anyhow::Result<AppConfig> {
    let text = std::fs::read_to_string("config.json")?;
    let mut root: serde_json::Value = serde_json::from_str(&text)?;
    if let Ok(overlay_text) = std::fs::read_to_string("/sandbox/config.json") {
        if let Ok(overlay) = serde_json::from_str::<serde_json::Value>(&overlay_text) {
            hs_utils::config::deep_merge(&mut root, overlay);
        }
    }
    hs_utils::config::prepare_config(&mut root);
    hs_utils::config::apply_env_overrides(&mut root);
    Ok(serde_json::from_value(root)?)
}
