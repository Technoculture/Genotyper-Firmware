use log::info;
use workflow::conf::generate_json_schemas;

use simplelog::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the logger
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        // LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    info!("Starting schemagen");
    generate_json_schemas()?;
    info!("Finished schemagen");

    Ok(())
}
