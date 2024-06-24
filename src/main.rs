use std::{fs, io, process, thread, time::Duration};

mod config;

fn main() {
    let nologin_config = (|| -> Result<config::Nologin, io::Error> {
        let config_path = config::read().nologin_path;
        let config_str = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<config::Nologin>(&config_str).unwrap_or_default())
    })()
    .unwrap_or_default();

    println!("{}", nologin_config.text);

    thread::sleep(Duration::from_secs(nologin_config.logout_timeout));
    process::exit(0);
}
