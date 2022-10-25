// #[macro_use] extern crate log;
// extern crate simplelog;
use simplelog::*;
extern crate shellexpand;

use clap::Parser;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use actix_web::{App, HttpServer, Responder, web, get};
use keri::daemon::DaemonConfig;

/// command line arguments for the KERI Daemon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct KERIDaemonArgs {
    /// Sets a custom config file. Defaults to $HOME/.keri/kerid.json
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Turn debug logging on or off
    #[arg(short, long)]
    debug: Option<bool>,
}

#[actix_web::main]
async fn main () -> std::io::Result<()> {
    let args = KERIDaemonArgs::parse();

    let log_level = match args.debug {
        Some(true) => LevelFilter::Debug,
        _ => LevelFilter::Info
    };
    CombinedLogger::init(
        vec![
            TermLogger::new(log_level, Config::default(),
                            TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    info!("{}", "starting...");


    let config_path = match args.config {
        Some(path) => path,
        None => Path::new(shellexpand::tilde("~/.keri/kerid.json").deref()).to_path_buf(),
    };

    debug!("arg: {:?}", &config_path);

    let config_file_data = match fs::read_to_string(&config_path) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to read daemon config file: {:?}, Error: {}", &config_path, e.to_string());
            std::process::exit(1);
        }
    };

    let config_json: DaemonConfig = serde_json::from_str(&config_file_data)
        .expect("config file data not well formed");

    debug!("Config file: {:?}", config_json);

    HttpServer::new(|| {
        App::new().service(kerinew)
    })
        .bind((config_json.host, config_json.port))?
        .run()
        .await
}

#[get("/keri/{name}/new")]
async fn kerinew(name: web::Path<String>) -> impl Responder {
    format!("NEW KERI for {name}")
}