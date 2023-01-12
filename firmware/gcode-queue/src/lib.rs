trait Gcode {
    fn gcode(&self) -> String;
}

// todo: change u32 to Position type
pub enum GcodeOp {
    GoHome,
    GotoX {x: u32, mm_per_min: u32},
    GotoY {y: u32, mm_per_min: u32},
    GotoXY {x: u32, y: u32, mm_per_min: u32},
    MotorOff
}

impl Gcode for GcodeOp {
    fn gcode(&self) -> String {
        match self {
            GcodeOp::GoHome => "G28".into(),
            GcodeOp::GotoX {x, mm_per_min} => {
                format!("G1 X{} F{}", x, mm_per_min)
            },
            GcodeOp::GotoY {y, mm_per_min} => {
                format!("G01 Y{} F{}", y, mm_per_min)
            },
            GcodeOp::GotoXY {x, y, mm_per_min} => {
                format!("G01 X{} Y{} F{}", x, y, mm_per_min)
            },
            GcodeOp::MotorOff => "M84".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_to_gcode() {
        assert_eq!(GcodeOp::GoHome.gcode(), String::from("G28"));
    }
}
