#![allow(non_snake_case)]
use std::{fmt::Display, path::PathBuf};

use crate::display::menuopts::CUTE_LOGO;

use database::db::DB;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use tui::style::Style;

// Application.
pub mod app;

// Database
pub mod database;

// Structures And Functions That Represent Screen Data
pub mod screens;

// Structures That Represent Display Items
pub mod display;

// Structures And Functions That Represent A CURL or WGET Request
pub mod request;

// Events & Event Handler
pub mod events;

pub mod tui_cute;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    colors: Colors,
    logo: Option<Logo>,
    db_path: Option<PathBuf>,
}

impl Config {
    pub fn get_fg_color(&self) -> tui::style::Color {
        self.colors.get_fg()
    }
    pub fn set_db_path(&mut self, path: PathBuf) {
        self.db_path = Some(path);
    }
    pub fn get_bg_color(&self) -> tui::style::Color {
        self.colors.get_bg()
    }
    pub fn get_body_color(&self) -> tui::style::Color {
        self.colors.body.get_value()
    }
    pub fn get_outline_color(&self) -> tui::style::Color {
        self.colors.outline.get_value()
    }
    pub fn get_logo(&self) -> &str {
        if self.logo == Some(Logo::Default) {
            CUTE_LOGO
        } else {
            ""
        }
    }

    pub fn get_style(&self) -> Style {
        Style::default()
            .fg(self.get_fg_color())
            .bg(self.get_bg_color())
    }

    pub fn get_style_error(&self) -> Style {
        Style::default()
            .fg(tui::style::Color::Red)
            .bg(self.get_bg_color())
    }

    pub fn get_default_config() -> Self {
        Self {
            colors: Colors {
                fg: ConfigColor::Gray,
                bg: ConfigColor::Black,
                body: ConfigColor::Yellow,
                outline: ConfigColor::Cyan,
            },
            logo: Some(Logo::Default),
            db_path: Some(DB::get_default_path()),
        }
    }

    pub fn load() -> Result<Self, String> {
        if let Some(config) = config_dir() {
            let config = config.join("CuTE").join("config.toml");
            if let Ok(config) = std::fs::read_to_string(config) {
                if let Ok(config) = toml::from_str::<Config>(&config) {
                    Ok(config)
                } else {
                    Err("Failed to parse config.toml".to_string())
                }
            } else {
                Err("Failed to read config.toml".to_string())
            }
        } else {
            Err("Failed to get config directory".to_string())
        }
    }

    pub fn get_db_path(&self) -> Option<PathBuf> {
        self.db_path.as_ref().cloned()
    }
}
impl Default for Config {
    fn default() -> Self {
        if let Ok(config) = Self::load() {
            config
        } else {
            Self::get_default_config()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Logo {
    #[default]
    Default,
    None,
}

impl Display for Logo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logo::Default => write!(f, "{}", CUTE_LOGO),
            Logo::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colors {
    fg: ConfigColor,
    bg: ConfigColor,
    body: ConfigColor,
    outline: ConfigColor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ConfigColor {
    Red,
    Blue,
    Cyan,
    Magenta,
    Gray,
    Black,
    White,
    Green,
    Yellow,
}

impl Colors {
    pub fn get_fg(&self) -> tui::style::Color {
        self.fg.get_value()
    }
    pub fn get_bg(&self) -> tui::style::Color {
        self.bg.get_value()
    }
}

impl ConfigColor {
    pub fn get_value(&self) -> tui::style::Color {
        match self {
            ConfigColor::Red => tui::style::Color::Red,
            ConfigColor::Blue => tui::style::Color::Blue,
            ConfigColor::Cyan => tui::style::Color::Cyan,
            ConfigColor::Magenta => tui::style::Color::Magenta,
            ConfigColor::Gray => tui::style::Color::Gray,
            ConfigColor::Black => tui::style::Color::Black,
            ConfigColor::White => tui::style::Color::White,
            ConfigColor::Green => tui::style::Color::Green,
            ConfigColor::Yellow => tui::style::Color::Yellow,
        }
    }
}
