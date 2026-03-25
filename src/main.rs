mod spicetify;
mod wal;

use clap::{Arg, Command};
use std::env;

use spicetify::Spicetify;
use wal::Wal;

fn main() {
    let matches = Command::new("pywal-spicetify")
        .version("0.1")
        .about("A simple cli tool for setting spicetify colors from wal")
        .arg(
            Arg::new("theme")
                .help("Path to wallpaper image")
                .required(true),
        )
        .arg(
            Arg::new("reset")
                .short('r')
                .long("reset")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("saturate")
                .long("saturate")
                .value_name("FLOAT")
                .value_parser(clap::value_parser!(f32))
                .help("Saturation level for pywal"),
        )
        .arg_required_else_help(true)
        .get_matches();

    let theme: String = matches
        .get_one::<String>("theme")
        .expect("theme required")
        .to_string();

    let saturate: Option<String> = matches.get_one::<f32>("saturate").map(|v| v.to_string());

    println!("Wallpaper: {theme}");
    if let Some(ref sat) = saturate {
        println!("Using saturation: {sat}");
    }

    // Only linux support
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("unable to locate home directory!"),
    };

    let wal = Wal::new(home.clone(), saturate, theme.clone());
    let spicetify = Spicetify::new(home.clone(), &theme);

    if matches.get_flag("reset") {
        println!("Resetting configs...");
        wal.reset();
        spicetify.write_config(None);
        spicetify.reload();
    } else {
        wal.set_config();
        let wal_config = wal.get_config();

        spicetify.write_config(Some(wal_config));
        spicetify.reload();
    }
}
