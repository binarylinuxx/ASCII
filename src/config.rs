// src/config.rs
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolPack {
    Standard,
    Detailed,
    Blocks,
    Minimal,
    Digits,
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Colorful,
    Grayscale,
    Inverted,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Unknown symbol pack: {0}")]
    UnknownSymbolPack(String),
    
    #[error("Unknown render mode: {0}")]
    UnknownRenderMode(String),
}

impl SymbolPack {
    // Different symbol packs from darkest to lightest
    pub fn symbols(&self) -> &'static str {
        match self {
            SymbolPack::Standard => "@%#*+=-:. ",
            SymbolPack::Detailed => "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ",
            SymbolPack::Blocks => "█▓▒░ ",
            SymbolPack::Minimal => "#@&%=-:. ",
            SymbolPack::Digits => "9876543210 ",
            SymbolPack::Binary => "010101011110",
        }
    }
}

impl FromStr for SymbolPack {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(SymbolPack::Standard),
            "detailed" => Ok(SymbolPack::Detailed),
            "blocks" => Ok(SymbolPack::Blocks),
            "minimal" => Ok(SymbolPack::Minimal),
            "digits" => Ok(SymbolPack::Digits),
            "binary" => Ok(SymbolPack::Binary),
            _ => Err(ConfigError::UnknownSymbolPack(s.to_string())),
        }
    }
}

impl FromStr for RenderMode {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "colorful" => Ok(RenderMode::Colorful),
            "grayscale" => Ok(RenderMode::Grayscale),
            "inverted" => Ok(RenderMode::Inverted),
            _ => Err(ConfigError::UnknownRenderMode(s.to_string())),
        }
    }
}

// Configuration struct for the ASCII art generator
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: String,
    pub output_path: String,
    pub mode: RenderMode,
    pub symbols: SymbolPack,
    pub target_width: u32,
    pub target_height: u32,
    pub show_instantly: bool,
    pub auto_square: Option<String>,
}
