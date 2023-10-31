use chrono::prelude::*;
use chrono::Duration;
use log::debug;
use slider_driver::Position;
use std::fmt::{Display, Formatter};

/// ToGcode trait for converting a type to a Gcode string
pub trait ToGcode {
    fn gcode(&self) -> Vec<u8>;
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait GcodeInfo {
    fn code(&self) -> String;
    fn description(&self) -> String;
}

pub trait GcodeExecutable {
    fn execute(&mut self) -> Result<(), String>;
}

/// An Op (short for Operation) is a single line of GCode
#[derive(Debug, Clone)]
pub enum Op {
    GoHome,
    GotoX {
        x: Position,
        mm_per_min: u32,
    },
    GotoY {
        y: Position,
        mm_per_min: u32,
    },
    GotoXY {
        x: Position,
        y: Position,
        mm_per_min: u32,
    },
    MotorOff,
    Reset,
}

impl GcodeInfo for Op {
    /// Returns the GCode for the operation
    /// # Example
    /// ```
    /// use gantry_controller::{Op, GcodeInfo, ToGcode};
    /// use slider_driver::{Position, Range};
    /// static RANGE: Range = Range { min: 0, max: 100 };
    /// let op = Op::GotoX { x: Position::new(100, &RANGE), mm_per_min: 1000 };
    /// assert_eq!(op.gcode().unwrap(), String::from("G01 X100 F1000"));
    /// ```
    fn code(&self) -> String {
        match self {
            Op::GoHome => "G28".to_string(),
            Op::GotoX { .. } => "G01".to_string(),
            Op::GotoY { .. } => "G01".to_string(),
            Op::GotoXY { .. } => "G01".to_string(),
            Op::MotorOff => "M84".to_string(),
            Op::Reset => "M999".to_string(),
        }
    }

    /// Returns a description of the operation
    /// # Example
    /// ```
    /// use gantry_controller::{Op, GcodeInfo};
    /// use slider_driver::{Position, Range};
    /// static RANGE: Range = Range { min: 0, max: 100 };
    /// let op = Op::GotoX { x: Position::new(100, &RANGE), mm_per_min: 1000 };
    /// assert_eq!(op.description(), "Go to X");
    /// ```
    fn description(&self) -> String {
        match self {
            Op::GoHome => "Go Home".to_string(),
            Op::GotoX { .. } => "Go to X".to_string(),
            Op::GotoY { .. } => "Go to Y".to_string(),
            Op::GotoXY { .. } => "Go to XY".to_string(),
            Op::MotorOff => "Motor Off".to_string(),
            Op::Reset => "Reset".to_string(),
        }
    }
}

impl ToGcode for Op {
    /// Convert an Op to a GCode string
    fn gcode(&self) -> Vec<u8> {
        let code = self.code();
        let mut gc = match self {
            Op::GoHome => code,
            Op::GotoX { x, mm_per_min } => format!("{} X{} F{}", code, x.value(), mm_per_min),
            Op::GotoY { y, mm_per_min } => format!("{} Y{} F{}", code, y.value(), mm_per_min),
            Op::GotoXY { x, y, mm_per_min } => {
                format!("{} X{} Y{} F{}", code, x.value(), y.value(), mm_per_min)
            }
            Op::MotorOff => code,
            Op::Reset => code,
        };
        gc = format!("{}\r\n", gc);
        print!("Gcode: {}", gc);
        gc.as_bytes().to_vec()
    }

    /// Convert an Op to a GCode byte array
    fn as_bytes(&self) -> Vec<u8> {
        let gc = self.gcode();
        gc.to_vec()
    }
}

// pub type GcodeSequence = Vec<Op>;

// impl ToGcode for GcodeSequence {
/// Convert a GcodeSequence to a Gcode string
/// # Example
/// ```
/// use gantry_controller::{GcodeSequence, Op, ToGcode};
/// use slider_driver::{Position, Range};
///
/// static RANGE: Range = Range { min: 0, max: 100 };
/// let seq: GcodeSequence = vec![
///     Op::GoHome,
///     Op::GotoXY { x: Position::new(100, &RANGE), y: Position::new(100, &RANGE), mm_per_min: 1000 },
///     Op::MotorOff
/// ].into();
///
/// let gcode = seq.gcode().unwrap();
/// assert_eq!(gcode, "G28\nG01 X100 Y100 F1000\nM84\n");
/// ```
/// # Errors
/// Returns None if the GcodeSequence is empty
//     fn gcode(&self) -> Vec<u8> {
//         let mut gcode = String::new();
//         for op in self {
//             let op_gcode = op.gcode();
//             gcode.push_str(&op_gcode);
//             gcode.push_str("\n");
//         }
//         Some(gcode)
//     }

//     /// Convert a GcodeSequence to a Gcode byte array
//     fn as_bytes(&self) -> Vec<u8> {
//         let gc = self.gcode().unwrap();
//         gc.as_bytes().to_vec()
//     }
// }

/// Statistics related to Task execution
/// ```txt
///             ┌──────────┐
///             │created_at├───┐
///             └─┬────────┘   │
///               │start()     │
/// ┌────────┐  ┌─▼────────┐  ┌▼──────┐
/// │duration◄──┤started_at│  │elapsed│
/// └───────▲┘  └──────────┘  └▲──────┘
///         │complete()        │
///         │   ┌────────────┐ │
///         └───┤completed_at├─┘
///             └────────────┘
/// ```
#[derive(Debug, Clone)]
pub struct TaskStats {
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub elapsed: Option<Duration>,
}

impl TaskStats {
    /// Create a new TaskStats
    pub fn new() -> Self {
        Self {
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            duration: None,
            elapsed: None,
        }
    }

    /// Start the TaskStats
    pub fn start(&mut self) {
        self.started_at = Some(Utc::now());
    }

    /// Complete the TaskStats
    /// # Example
    /// ```
    /// use gantry_controller::TaskStats;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// let mut stats = TaskStats::new();
    /// stats.start();
    /// sleep(Duration::from_millis(100));
    /// stats.complete();
    /// assert!(stats.elapsed.unwrap().num_milliseconds() >= 100);
    /// ```
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.duration = self.started_at.map(|started_at| {
            self.completed_at
                .unwrap_or_else(Utc::now)
                .signed_duration_since(started_at)
        });
        self.elapsed = self.started_at.map(|created_at| {
            self.completed_at
                .unwrap_or_else(Utc::now)
                .signed_duration_since(created_at)
        });
    }
}

impl Display for TaskStats {
    /// Display the TaskStats
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let duration = self.duration.unwrap_or_else(|| Duration::seconds(0));
        let elapsed = self.elapsed.unwrap_or_else(|| Duration::seconds(0));
        write!(
            f,
            "TaskStats {{ duration: {}, elapsed: {} }}",
            duration,
            elapsed - duration
        )
    }
}

// A Task is a sequence of Gcode operations
// #[derive(Debug, Clone)]
// pub struct GcodeTask {
//     /// Name of the Task, used for logging
//     pub name: String,
//     /// The GcodeSequence to execute
//     pub seq: GcodeSequence,
//     /// Priority of the Task, used for sorting
//     pub priority: u8,
//     /// Statistics related to Task execution
//     pub stats: TaskStats,
// }

// impl GcodeTask {
//     /// Create a new GcodeTask
//     /// # Example
//     /// ```
//     /// use gantry_controller::{GcodeTask, GcodeSequence, Op, ToGcode};
//     /// use slider_driver::{Position, Range};
//     ///
//     /// static RANGE: Range = Range { min: 0, max: 100 };
//     /// let seq: GcodeSequence = vec![
//     ///    Op::GoHome,
//     ///    Op::GotoXY { x: Position::new(100, &RANGE), y: Position::new(100, &RANGE), mm_per_min: 1000 },
//     ///    Op::MotorOff
//     /// ].into();
//     ///
//     /// let task = GcodeTask::new(String::from("test"), seq, 1);
//     /// assert_eq!(task.name, "test");
//     /// assert_eq!(task.seq.len(), 3);
//     /// assert_eq!(task.priority, 1);
//     /// assert_eq!(task.stats.duration, None);
//     /// assert_eq!(task.stats.elapsed, None);
//     /// assert_eq!(task.stats.completed_at, None);
//     /// assert_eq!(task.gcode().unwrap(), "G28\nG01 X100 Y100 F1000\nM84\n");
//     /// ```
//     /// # Errors
//     /// Returns None if the GcodeSequence is empty
//     pub fn new(name: String, seq: GcodeSequence, priority: u8) -> Self {
//         Self {
//             name,
//             seq,
//             priority,
//             stats: TaskStats::new(),
//         }
//     }

//     /// Start the Task
//     pub fn start(&mut self) {
//         self.stats.start();
//     }

//     /// Complete the Task
//     pub fn complete(&mut self) {
//         self.stats.complete();
//     }
// }

// impl ToGcode for GcodeTask {
//     /// Convert the GcodeTask to Gcode
//     fn gcode(&self) -> Option<String> {
//         self.seq.gcode()
//     }

//     /// Convert the GcodeTask to Gcode bytes
//     fn as_bytes(&self) -> Vec<u8> {
//         self.seq.as_bytes()
//     }
// }

// impl GcodeExecutable for GcodeSequence {
//     /// Execute a GcodeSequence
//     fn execute(&mut self) -> Result<(), String> {
//         debug!("⏳ Executing GcodeSequence");
//         for op in self {
//             debug!("⏳ Executing {:?}", op);
//             debug!("⏳ Executing {}", op.gcode().expect("No Gcode Error"));
//         }
//         Ok(())
//     }
// }

// impl GcodeExecutable for GcodeTask {
//     /// Execute a GcodeTask
//     fn execute(&mut self) -> Result<(), String> {
//         debug!("⏳ Executing GcodeTask {}", self.name);
//         self.stats.start();
//         self.seq.execute().expect("Failed to execute GcodeSequence");
//         self.stats.complete();
//         debug!("⏱  GcodeTask {} stats: {}", self.name, self.stats);
//         Ok(())
//     }
// }

// Tests
// -----
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use slider_driver::Range;

//     #[test]
//     fn enum_to_gcode() {
//         static RANGE: Range = Range {
//             min: 0,
//             max: 10_000,
//         };

//         assert_eq!(Op::GoHome.gcode(), Some(String::from("G28")));
//         assert_eq!(
//             Op::GotoX {
//                 x: Position::new(10_000, &RANGE),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 X10000 F1000"))
//         );
//         assert_eq!(
//             Op::GotoY {
//                 y: Position::new(5, &RANGE),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 Y5 F1000"))
//         );
//         assert_eq!(
//             Op::GotoXY {
//                 x: Position::new(0, &RANGE),
//                 y: Position::new(11, &RANGE),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 X0 Y11 F1000"))
//         );
//         assert_eq!(Op::MotorOff.gcode(), Some(String::from("M84")));
//     }

//     #[test]
//     fn enum_to_gcode_outside_range() {
//         static RANGE1: Range = Range { min: 20, max: 100 };
//         static RANGE2: Range = Range { min: 0, max: 10 };

//         assert_eq!(
//             Op::GotoX {
//                 x: Position::new(10_000, &RANGE1),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 X100 F1000"))
//         );
//         assert_eq!(
//             Op::GotoY {
//                 y: Position::new(5, &RANGE1),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 Y20 F1000"))
//         );
//         assert_eq!(
//             Op::GotoXY {
//                 x: Position::new(0, &RANGE2),
//                 y: Position::new(11, &RANGE2),
//                 mm_per_min: 1000
//             }
//             .gcode(),
//             Some(String::from("G01 X0 Y10 F1000"))
//         );
//     }

//     #[test]
//     fn gcode_seq_to_gcode() {
//         static RANGE: Range = Range { min: 0, max: 1200 };

//         let seq: GcodeSequence = vec![
//             Op::GoHome,
//             Op::GotoXY {
//                 x: Position::new(10_000, &RANGE),
//                 y: Position::new(10_000, &RANGE),
//                 mm_per_min: 1000,
//             },
//             Op::MotorOff,
//         ]
//         .into();

//         assert_eq!(
//             seq.gcode().unwrap(),
//             String::from("G28\nG01 X1200 Y1200 F1000\nM84\n")
//         );
//     }

//     #[test]
//     fn taskstats() {
//         //use std::thread::sleep;
//         //use std::time::Duration;

//         let mut stats = TaskStats::new();
//         assert_eq!(stats.started_at, None);
//         assert_eq!(stats.completed_at, None);
//         assert_eq!(stats.duration, None);
//         assert_eq!(stats.elapsed, None);

//         stats.start();
//         let started_at = stats.started_at;
//         assert_eq!(stats.completed_at, None);
//         assert_eq!(stats.duration, None);
//         assert_eq!(stats.elapsed, None);
//         //assert!(stats.started_at.unwrap() < Utc::now());
//         //assert!(Utc::now() - stats.started_at.unwrap() < Duration::seconds(1));

//         //sleep(Duration::seconds(1));
//         stats.complete();
//         //assert!(Utc::now() - stats.completed_at.unwrap() > Duration::from_secs(2));
//         assert_eq!(stats.started_at, started_at);
//         //assert_eq!(stats.duration, Some(Duration::from_secs(0)));
//         //assert_eq!(stats.elapsed, Some(Duration::from_secs(0)));

//         //todo!("Finish TaskStats tests");
//     }
// }
