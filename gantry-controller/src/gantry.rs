// let mut gantry = Gantry::new(port);
// gantry.reset().await.unwrap();
// gantry.home().await.unwrap();
// gantry.move_to(0.0, 0.0).await.unwrap();
// gantry.move_to(100.0, 100.0).await.unwrap();

use crate::{GcodeInfo, ToGcode};

use super::gcode::Op;
use slider_driver::{Position, Range};
use tokio_serial::SerialPort;

pub struct Gantry {
    port: Box<dyn SerialPort>,
}

#[derive(Debug)]
pub enum GantryError {
    SerialError(tokio_serial::Error),
}

impl Gantry {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }

    // pub fn reset(&mut self) -> Result<(), GantryError> {
    //     let message = Op::Reset.gcode().as_slice();
    //     self.port.write(message).unwrap();
    //     self.port.flush().unwrap();
    //     Ok(())
    // }

    pub fn home(&mut self) -> Result<(), GantryError> {
        let message = Op::GoHome.gcode();
        // println!("{:?}", message.to_ascii_lowercase());
        self.port.write(message.as_slice()).unwrap();
        self.port.flush().unwrap();
        Ok(())
    }

    pub fn move_to(&mut self, x: u32, y: u32) -> Result<(), GantryError> {
        static RANGE: Range = Range { min: 0, max: 1200 };
        let cmd = Op::GotoXY {
            x: Position::new(x, &RANGE),
            y: Position::new(y, &RANGE),
            mm_per_min: 2000,
        }
        .gcode();
        // println!("{:?}", cmd.to_ascii_lowercase());
        self.port.write(cmd.as_slice()).unwrap();
        self.port.flush().unwrap();
        Ok(())
    }
}
