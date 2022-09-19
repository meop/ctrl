// use std::sync::Arc;
// use std::sync::Mutex;
// use std::sync::RwLock;

// use serde::Deserialize;

// use config::Config;

// enum PkgTool {
//     Apt,
//     Pacman,
//     Homebrew,
//     Winget,
// }

// #[derive(Deserialize)]
// struct PkgConfig {
//     tool: PkgTool,
// }

// struct Cfg {
//     pkg: PkgConfig,
// }

// fn load()

// static mut SETTINGS: Arc<RwLock<Config>> = Arc::new(RwLock::new(Config::default()));

// fn get(key: &String) -> Result<(), std::io::Error> {

// }

// fn set<T>(key: &String, val: &T) -> Result<(), std::io::Error> {}
