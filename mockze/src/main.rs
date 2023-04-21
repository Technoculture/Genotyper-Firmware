use workflow::{get_tree_by_name, get_workflow_by_title, root_library_path, load_library};
use log::{info, error};
use simplelog::*;

const LOG_FILE: &str = "/tmp/tcr/mockze.log";

fn main() {
    // Initialize logger
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            // LevelFilter::Info, 
            Config::default(), 
            TerminalMode::Mixed,
            ColorChoice::Auto),
        WriteLogger::new(
            LevelFilter::Info, 
            Config::default(), 
            std::fs::File::create(LOG_FILE).unwrap()),
    ]).unwrap();

    info!("Starting mockze");

    let library_path = root_library_path().expect("Could not find root library path");
    let library = load_library(&library_path).expect("Could not load library");

    println!("Modules: {:#?}", library.modules);

    // 1. Get the list of modules and their APIs
    let modules = library.modules;
    for module in modules.content {
        let api = &module.api;
        println!("API for module {}: {:#?}", module.info.name, api);
    }

    // 2. For every service in the API spec, create a mock service

    // 3. Start all the mock services in seperate processes
}
