// #![allow(unused_imports)]
// #![allow(dead_code)]

use gantry_controller::{find_gantry_port_name, open_gantry_port};

#[tokio::main]
async fn main() {
    let gantry = find_gantry_port_name();
    let port = open_gantry_port(&gantry.port_name);

    // let mut gantry = Gantry::new(port);
    // gantry.reset().await.unwrap();
    // gantry.home().await.unwrap();
    // gantry.move_to(0.0, 0.0).await.unwrap();
    // gantry.move_to(100.0, 100.0).await.unwrap();

    println!("Port opened: {:?}", port.name());
}
