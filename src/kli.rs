use simplelog::*;

fn main () {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(),
                            TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    info!("kli executing...");
}