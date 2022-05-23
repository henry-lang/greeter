mod config;
mod widget;

use std::path::PathBuf;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load(
        [
            home::home_dir().expect("home directory"),
            ".greeter.toml".into(),
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    println!(
        "{}",
        match username::get_user_name() {
            Ok(name) => {
                format!("Greetings, {}!", name)
            }
            _ => {
                "Greetings!".into()
            }
        }
    );

    Ok(())
}
