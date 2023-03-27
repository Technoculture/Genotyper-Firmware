// #![allow(unused_imports)]
// #![allow(dead_code)]

use gantry_controller::{find_gantry_port_name, open_gantry_port, Gantry};

#[tokio::main]
async fn main() {
    let gantry = find_gantry_port_name();
    let port = open_gantry_port(&gantry.port_name);
    println!("Port opened: {:?}", port.name());

    let mut gantry = Gantry::new(port);
    gantry.home().expect("Did not trigger homing");

    for _ in 0..10 {
        gantry.move_to(100, 100).expect("Did not move to 100, 100");
        gantry.move_to(10, 90);
        gantry.move_to(110, 80);
    }

    gantry.move_to(100, 100).expect("Did not move to 100, 100");
    gantry.move_to(10, 90);
    gantry.move_to(110, 80);

    println!("Done");
}
