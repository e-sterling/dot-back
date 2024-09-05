pub mod backup {

    use std::borrow::Borrow;

    use crate::config_handler::AppConfig;
    use rustypath::RPath;

    pub fn run(config: AppConfig) {
        
    }

    fn copy_and_check(src: RPath, dst: RPath) {
        let v: bool = true;

        let home_dir: RPath = RPath::gethomedir();
        let tmp_dir: RPath = home_dir.join("tmp");
        let known_dot_files: Vec<&str> = vec![".zshrc", ".vimrc", ".tmux.conf"];
        let mut matching_files: Vec<RPath> = Vec::new(); // Vector to store paths of matching files

        println!("Examining directory {}...", src.convert_to_string());

        for entry in src
            .read_dir()
            .expect("Failed to read contents of source directory")
        {
            if let Ok(entry) = entry {
                let file_name: std::ffi::OsString = entry.file_name();
                if known_dot_files.contains(&file_name.to_string_lossy().as_ref()) {
                    matching_files.push(RPath::from(entry.path()));
                    if v {
                        println!(
                            "Found: {}",
                            <std::ffi::OsString as Into<std::path::PathBuf>>::into(file_name)
                                .to_string_lossy()
                                .to_string()
                        )
                    }
                }
            }
        }
    }
}

pub mod config_handler {
    use figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment,
    };
    use rustypath::RPath;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AppConfig {
        backup_dest: String,
        backup_type: String,
    }

    impl Default for AppConfig {
        fn default() -> AppConfig {
            AppConfig {
                backup_dest: RPath::gethomedir().convert_to_string(),
                backup_type: "file".into(),
            }
        }
    }

    pub fn load() -> Result<AppConfig, figment::Error> {
        let figment = Figment::from(Serialized::defaults(AppConfig::default()))
            .merge(Toml::file("AppConfig.toml"))
            .merge(Env::prefixed("DOTBACK_"));

        let config: AppConfig = figment.extract()?;
        Ok(config)
    }
}
