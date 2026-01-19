use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_THRESHOLD: f32 = 0.10;
const DEFAULT_MIN_FINGERS: i32 = 3;
const DEFAULT_EDGE_MARGIN: f32 = 0.0;
const DEFAULT_MIN_PRESSURE: f32 = 0.0;
const DEFAULT_TWO_FINGER_COOLDOWN_MS: u64 = 0;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_threshold")]
    pub left: f32,
    #[serde(default = "default_threshold")]
    pub right: f32,
    #[serde(default = "default_threshold")]
    pub up: f32,
    #[serde(default = "default_threshold")]
    pub down: f32,
    #[serde(default = "default_min_fingers")]
    pub min_fingers: i32,
    #[serde(default = "default_edge_margin")]
    pub edge_margin: f32,
    #[serde(default = "default_min_pressure")]
    pub min_pressure: f32,
    #[serde(default = "default_two_finger_cooldown_ms")]
    pub two_finger_cooldown_ms: u64,
}

fn default_threshold() -> f32 {
    DEFAULT_THRESHOLD
}

fn default_min_fingers() -> i32 {
    DEFAULT_MIN_FINGERS
}

fn default_edge_margin() -> f32 {
    DEFAULT_EDGE_MARGIN
}

fn default_min_pressure() -> f32 {
    DEFAULT_MIN_PRESSURE
}

fn default_two_finger_cooldown_ms() -> u64 {
    DEFAULT_TWO_FINGER_COOLDOWN_MS
}

impl Default for Config {
    fn default() -> Self {
        Self {
            left: DEFAULT_THRESHOLD,
            right: DEFAULT_THRESHOLD,
            up: DEFAULT_THRESHOLD,
            down: DEFAULT_THRESHOLD,
            min_fingers: DEFAULT_MIN_FINGERS,
            edge_margin: DEFAULT_EDGE_MARGIN,
            min_pressure: DEFAULT_MIN_PRESSURE,
            two_finger_cooldown_ms: DEFAULT_TWO_FINGER_COOLDOWN_MS,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match Self::config_path() {
            Some(path) if path.exists() => {
                match fs::read_to_string(&path) {
                    Ok(contents) => {
                        match toml::from_str(&contents) {
                            Ok(config) => config,
                            Err(e) => {
                                log::warn!("Failed to parse config file: {}. Using defaults.", e);
                                Self::default()
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to read config file: {}. Using defaults.", e);
                        Self::default()
                    }
                }
            }
            _ => Self::default(),
        }
    }

    fn config_path() -> Option<PathBuf> {
        let home = dirs::home_dir()?;
        Some(home.join(".config/macos-on-swipe/config.toml"))
    }
}
