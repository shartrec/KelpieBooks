/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::Deserialize;
use std::{env, fs};
use std::path::PathBuf;
use std::sync::OnceLock;
use log::info;
use rocket::fs::relative;

#[derive(Deserialize, Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub from: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub base_url: String,
    pub default_locale: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub app: AppConfig,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config_dir() -> PathBuf {
    if let Ok(current_dir) = env::current_dir() {
        current_dir.join("config.toml")
    } else {
        panic!("failed to get current directory");
    }
}

fn get_default_config() -> PathBuf {
    relative!("config.toml").into()
}

pub fn load_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let path = get_config_dir();
        let config_str = if let Ok(config_str) = fs::read_to_string(path) {
            config_str
        } else {
            let path = get_default_config();
            if let Ok(config_str) = fs::read_to_string(path) {
                config_str
            } else {
                panic!("failed to read config.toml");
            }
        };
        toml::from_str(&config_str).expect("Failed to parse config.toml")
    })
}
