//! Slider State Machine
//!
//! Diagram:
//! ```none
//!  o       
//!  │      ┌┐Stop
//! ┌▼──────┴▼┐
//! │IdleAt(0)│
//! └▲──┬─────┘              
//!  │  │Goto(to)            ┌──┐
//!  │ ┌▼────────────────────▼─┐│Goto(to)
//!  │ │MoveTo(from,current,to)├┘
//!  │ └──────┬───┬────────────┘
//!  │Goto(0) │   │Stop
//! ┌┴────────▼┐ ┌▼──────────────┐
//! │IdleAt(to)│ │IdleAt(current)│
//! └─┬─▲──────┘ └───────────────┘
//!   └─┘Stop
//! ```
//!
//! Code Walkthrough: https://www.loom.com/share/3c9e1b622508449ebd2c9b87f7a962bd

#![no_std]
#![warn(missing_docs)] // Warn if there are missing docs
use core::fmt::{Debug, Formatter};

mod toggle;

/// Range is a range of values denoting the minimum and maximum values of the slider.
#[derive(Debug, PartialEq)]
pub struct Range {
    /// min is the minimum value of the range
    pub min: u32,
    /// max is the maximum value of the range
    pub max: u32,
}

/// Position is a value between min and max.
/// It is used to represent the positions of the slider.
#[derive(PartialEq, Clone)]
pub struct Position {
    value: u32,
    range: &'static Range,
}

impl Position {
    /// Create a new position
    /// If the value is outside the range, the position is set to the closest
    /// value in the range
    pub fn new(value: u32, range: &'static Range) -> Self {
        if value > range.max {
            return Self {
                value: range.max,
                range,
            };
        } else if value < range.min {
            return Self {
                value: range.min,
                range,
            };
        }
        Self { value, range }
    }

    /// When adding to a Position, the value is added to the position's value.
    fn add(self, value: u32) -> Self {
        Self::new(self.value + value, self.range)
    }

    /// When subtracting from a Position, the value is subtracted from the position's value.
    fn sub(self, value: u32) -> Self {
        Self::new(self.value - value, self.range)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// When comparing two positions, only the value is compared. The range is ignored.
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// Slider is a state machine that can be in one of two states:
/// IdleAt(position) or MoveTo(from, current, to)
/// The slider can be moved to a new position by calling act()
/// The slider can be ticked by calling tick()
/// The slider can be queried for its current position by calling position()
/// The slider can be queried for its destination by calling destination()
#[derive(Debug, PartialEq)]
pub struct Slider {
    /// state is the current state of the slider
    state: SliderState,
    /// range is the range of the slider
    range: &'static Range,
    /// tick_handler is a function that is called when the slider is ticked.
    tick_handler: fn(SliderState) -> SliderState,
}

/// SliderAction is an action that can be performed on the slider
#[derive(Debug, PartialEq)]
pub enum SliderAction {
    /// Stop the slider
    Stop,
    /// Go to position x
    Goto(Position),
}

/// Slider is a state machine that can be in one of two states:
/// IdleAt(position) or MoveTo(from, current, to)
#[derive(Debug, PartialEq, Clone)]
pub enum SliderState {
    /// Sldier is idle at position x
    IdleAt {
        /// position is the position the slider is idle at
        position: Position,
    },
    /// Slider is moving from postion 'from' to position 'to'
    MoveTo {
        /// from is the position the slider started moving from
        from: Position,
        /// current is the current position of the slider
        current: Position,
        /// to is the position the slider is moving to
        to: Position,
    },
}

impl Slider {
    /// Create a new slider
    /// The slider is initially idle at position 0
    /// Note: "Position 0" simply maps to the minimum value of its range
    pub fn new(range: &'static Range) -> Self {
        Slider {
            state: SliderState::IdleAt {
                position: Position::new(0, range),
            },
            range,
            tick_handler: Self::default_tick_handler,
        }
    }

    /// Get the current position of the slider
    /// If the slider is idle, the current position is the target position
    pub fn position(&self) -> Position {
        match &self.state {
            SliderState::IdleAt { position } => position.clone(),
            SliderState::MoveTo { current, .. } => current.clone(),
        }
    }

    /// Get the target position of the slider
    /// If the slider is idle, the target position is the current position
    pub fn destination(&self) -> Option<Position> {
        match &self.state {
            SliderState::IdleAt { .. } => None,
            SliderState::MoveTo { to, .. } => Some(to.clone()),
        }
    }

    /// Perform an action on the slider
    pub fn act(&mut self, action: SliderAction) -> &mut Self {
        self.state = match action {
            SliderAction::Stop => SliderState::IdleAt {
                position: self.position(),
            },
            SliderAction::Goto(x) => SliderState::MoveTo {
                from: self.position(),
                current: self.position(),
                to: x,
            },
        };
        self
    }

    /// Set the tick handler
    /// The tick handler is a function that is called every time the slider is ticked
    /// The tick handler is responsible for moving the slider one step closer to its destination
    /// The default tick handler is used if no tick handler is set
    /// The tick handler is passed the current state of the slider
    /// The tick handler is expected to return the new state of the slider
    pub fn set_tick_handler(&mut self, tick_handler: fn(SliderState) -> SliderState) -> &mut Self {
        self.tick_handler = tick_handler;
        self
    }

    /// Update the slider
    /// This will move the slider one step closer to its destination
    /// If the slider is idle, this function does nothing
    pub fn tick(&mut self) -> &mut Self {
        self.state = (self.tick_handler)(self.state.clone());
        self
    }

    /// Handle a tick of the slider
    /// This will move the slider one step closer to its destination
    /// If the slider is idle, this function does nothing
    fn tick_(from: Position, current: Position, to: Position) -> SliderState {
        let pos_delta: u32 = 1;
        if current == to {
            return SliderState::IdleAt { position: to };
        }

        let current_: Position;
        if current < to {
            current_ = current.add(pos_delta);
        } else {
            current_ = current.sub(pos_delta);
        }

        SliderState::MoveTo {
            from,
            current: current_,
            to,
        }
    }

    fn default_tick_handler(state: SliderState) -> SliderState {
        match state {
            SliderState::IdleAt { .. } => state,
            SliderState::MoveTo { from, current, to } => {
                Self::tick_(from.clone(), current.clone(), to.clone())
            }
        }
    }
}

/// Tests
/// -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_new_range_clamp() {
        static RANGE: Range = Range { min: 5, max: 10 };
        let pos = Position::new(0, &RANGE);
        assert_eq!(
            pos,
            Position {
                value: 5,
                range: &RANGE
            }
        );

        let pos = Position::new(15, &RANGE);
        assert_eq!(
            pos,
            Position {
                value: 10,
                range: &RANGE
            }
        );

        let pos = Position::new(100, &RANGE);
        assert_eq!(
            pos,
            Position {
                value: 10,
                range: &RANGE
            }
        );
    }

    /// Test if the slider when stopped multiple time, it stops just once
    #[test]
    fn test_slider_stop() {
        static RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);
        slider.act(SliderAction::Goto(Position {
            value: 50,
            range: &RANGE,
        }));

        for _ in 0..10 {
            slider.tick();
        }

        slider.act(SliderAction::Stop);
        assert_eq!(
            slider.state,
            SliderState::IdleAt {
                position: Position {
                    value: 10,
                    range: &RANGE
                }
            }
        );
        slider.act(SliderAction::Stop);
        assert_eq!(
            slider.state,
            SliderState::IdleAt {
                position: Position {
                    value: 10,
                    range: &RANGE
                }
            }
        );
    }

    /// Test that the slider state machine works
    #[test]
    fn test_slider() {
        static RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        assert_eq!(
            slider.position(),
            Position {
                value: 0,
                range: &RANGE
            }
        );
        assert_eq!(slider.destination(), None);

        slider.act(SliderAction::Goto(Position::new(50, &RANGE)));

        slider.tick().tick().tick();

        assert_eq!(
            slider.position(),
            Position {
                value: 3,
                range: &RANGE
            }
        );
        assert_eq!(
            slider.destination(),
            Some(Position {
                value: 50,
                range: &RANGE
            })
        );

        slider.act(SliderAction::Stop);
        assert_eq!(
            slider.position(),
            Position {
                value: 3,
                range: &RANGE
            }
        );
        assert_eq!(slider.destination(), None);
    }

    #[test]
    fn states_and_actions_work() {
        static RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(
            Slider::new(&RANGE).state,
            SliderState::IdleAt {
                position: Position {
                    value: 0,
                    range: &RANGE
                }
            }
        );
        assert_eq!(
            Slider::new(&RANGE).act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position {
                    value: 0,
                    range: &RANGE
                }
            }
        );
        assert_eq!(
            Slider::new(&RANGE).act(SliderAction::Goto(Position {
                value: 50,
                range: &RANGE
            })),
            &Slider {
                state: SliderState::MoveTo {
                    from: Position {
                        value: 0,
                        range: &RANGE
                    },
                    current: Position {
                        value: 0,
                        range: &RANGE
                    },
                    to: Position {
                        value: 50,
                        range: &RANGE
                    },
                },
                range: &RANGE,
                tick_handler: Slider::default_tick_handler,
            }
        );

        let mut slider = Slider::new(&RANGE);
        slider
            .act(SliderAction::Goto(Position {
                value: 50,
                range: &RANGE,
            }))
            .act(SliderAction::Goto(Position {
                value: 100,
                range: &RANGE,
            }));

        for _ in 0..150 {
            slider.tick();
        }
        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position {
                    value: 100,
                    range: &RANGE
                }
            }
        );
    }

    #[test]
    fn position_works() {
        static RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        assert_eq!(
            Slider::new(&RANGE).position(),
            Position {
                value: 0,
                range: &RANGE
            }
        );
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position::new(100, &RANGE)))
                .position(),
            Position {
                value: 0,
                range: &RANGE
            }
        );

        slider
            .act(SliderAction::Goto(Position::new(10, &RANGE)))
            .act(SliderAction::Stop)
            .act(SliderAction::Goto(Position::new(20, &RANGE)));
        for _ in 0..100 {
            slider.tick();
        }

        assert_eq!(
            slider.act(SliderAction::Stop).position(),
            Position {
                value: 20,
                range: &RANGE
            }
        );
    }

    #[test]
    fn destination_works() {
        static RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(Slider::new(&RANGE).destination(), None);
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position::new(10, &RANGE)))
                .act(SliderAction::Stop)
                .destination(),
            None
        );
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position::new(10, &RANGE)))
                .act(SliderAction::Goto(Position::new(20, &RANGE)))
                .destination(),
            Some(Position {
                value: 20,
                range: &RANGE
            })
        );
    }

    #[test]
    fn act_works() {
        static RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        slider
            .act(SliderAction::Goto(Position::new(10, &RANGE)))
            .act(SliderAction::Goto(Position::new(20, &RANGE)))
            .act(SliderAction::Goto(Position::new(30, &RANGE)));

        for _ in 0..60 {
            slider.tick();
        }

        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position {
                    value: 30,
                    range: &RANGE
                }
            }
        );
    }

    #[test]
    fn new_works() {
        static RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(
            Slider::new(&RANGE).state,
            SliderState::IdleAt {
                position: Position {
                    value: 0,
                    range: &RANGE
                }
            }
        );
    }
}
