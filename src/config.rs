use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub icon_size: i32,
    pub spacing: i32,
    pub apps: Vec<String>,
}
