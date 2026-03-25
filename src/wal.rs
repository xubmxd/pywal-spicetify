use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process;

pub struct Wal {
    config_path: PathBuf,
    cache_path: PathBuf,
    saturate: Option<String>,
    wallpaper: String,
}

impl Wal {
    pub fn new(home: PathBuf, saturate: Option<String>, wallpaper: String) -> Self {
        let mut config_path = home.clone();
        let mut cache_path = home.clone();

        config_path.push(".config/wal/templates/colors-spicetify.ini");
        cache_path.push(".cache/wal/colors-spicetify.ini");

        Wal {
            config_path,
            cache_path,
            saturate,
            wallpaper,
        }
    }

    pub fn reload(&self) {
        let mut cmd = process::Command::new("wal");

        cmd.arg("-i").arg(&self.wallpaper);

        if let Some(sat) = &self.saturate {
            cmd.arg("--saturate").arg(sat);
        }

        cmd.output().expect("Failed to run wal");
    }

    pub fn reset(&self) {
        println!("Resetting...");
        let files = [&self.config_path, &self.cache_path];
        let mut existing_files = files.into_iter().filter(|&path| path.exists()).peekable();
        let do_files_exist = existing_files.peek().is_some();

        if !do_files_exist {
            println!(
                "Files {} and {} have been removed",
                self.config_path.display(),
                self.cache_path.display()
            );
            return;
        }

        for path in existing_files {
            println!("Removing file {}", path.display());
            if let Err(error) = std::fs::remove_file(path) {
                println!("Failed to remove config: {error}");
            }
        }

        self.reload();
    }

    pub fn set_config(&self) {
        let path = &self.config_path;

        if !path.exists() {
            let mut file = match OpenOptions::new().create(true).write(true).open(&path) {
                Ok(f) => f,
                Err(e) => panic!("Error opening .config/wal {}", e),
            };

            println!(
                "Generating colors-spicetify.ini file in {}",
                &path.display()
            );

            let content = r#"accent             = {color0.strip}
accent-active      = {color2.strip}
accent-inactive    = {color3.strip}
banner             = {color4.strip}
border-active      = {foreground.strip}
border-inactive    = {foreground.strip}
header             = {foreground.strip}
highlight          = {color6.strip}
main               = {background.strip}
notification       = {color7.strip}
notification-error = {color8.strip}
subtext            = {cursor.strip}
text               = {cursor.strip}"#;

            let _ = file.write_all(content.as_bytes());
        }

        // 🔥 Always regenerate wal (with optional saturation)
        self.reload();
    }

    pub fn get_config(&self) -> String {
        let path = &self.cache_path;

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening colors-spicetify.ini! {}", e),
        };

        println!("wal config path: {}", path.display());

        let mut reader = BufReader::new(file);
        let mut wal_config = String::new();
        let _ = reader.read_to_string(&mut wal_config);

        wal_config
    }
}
