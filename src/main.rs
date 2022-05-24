mod config;
mod widget;

use std::path::PathBuf;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load(
        [
            home::home_dir().expect("home directory"),
            ".greeter.json".into(),
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    if config.debug {
        dbg!(&config);
    }

    for widget in config.widgets {
        widget.display();
    }

    Ok(())
}
