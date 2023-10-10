#![allow(non_snake_case)]
// Purpose: Main library file for the application.
// ********************************************************************
use crate::display::menuopts::CUTE_LOGO;
use serde::{Deserialize, Serialize};

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
    logo: Logo,
    db_path: String,
}
impl Config {
    pub fn get_fg_color(&self) -> tui::style::Color {
        self.colors.get_fg()
    }
    pub fn get_bg_color(&self) -> tui::style::Color {
        self.colors.get_bg()
    }
    pub fn get_logo(&self) -> &Logo {
        &self.logo
    }
    pub fn get_db_path(&self) -> &String {
        &self.db_path
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors {
                fg: ConfigColor::Cyan,
                bg: ConfigColor::Gray,
            },
            logo: Logo::Logo1(String::from(CUTE_LOGO)),
            db_path: String::from(
                dirs::data_local_dir()
                    .expect("Failed to get local data directory")
                    .join("CuTE")
                    .into_os_string()
                    .to_str()
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Logo {
    Logo1(String),
    Logo2(String),
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colors {
    fg: ConfigColor,
    bg: ConfigColor,
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
        }
    }
}
