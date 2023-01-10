//! Slider State Machine
//!
//! Diagram:
//! ```none
//!  o       ┌─┐
//!  │   Stop│ │
//! ┌▼───────┴─▼┐
//! │ IdleAt(0) │
//! └▲──┬───────┘
//!  │  │Goto(to)
//!  │ ┌▼────────────────────────┐
//!  │ │ MoveTo(from,current,to) │
//!  │ └────────┬───┬────────────┘
//!  │Goto(0)   │   │Stop
//! ┌┴──────────▼┐ ┌▼───────────────┐
//! │ IdleAt(to) │ │ IdleAt(current)│
//! └─┬─▲────────┘ └────────────────┘
//!   │ │Stop
//!   └─┘
//! ```

#![no_std]
use core::fmt::{Debug, Formatter};

#[derive(Debug, PartialEq)]
pub struct Range {
    pub min: u32,
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

#[derive(Debug, PartialEq)]
pub struct Slider {
    state: SliderState,
    range: &'static Range,
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
    IdleAt { position: Position },
    /// Slider is moving from postion 'from' to position 'to'
    MoveTo {
        from: Position,
        current: Position,
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

    /// Update the slider
    /// This will move the slider one step closer to its destination
    /// If the slider is idle, this function does nothing
    fn tick_(&mut self, from: Position, current: Position, to: Position) -> SliderState {
        let pos_delta: u32 = 1;
        if current == to {
            return SliderState::IdleAt { position: to };
        }

        let current_: Position;
        if current < to {
            current_ = current.clone().add(pos_delta);
        } else {
            current_ = current.clone().sub(pos_delta);
        }

        SliderState::MoveTo {
            from: from,
            current: current_,
            to: to,
        }
    }

    /// Update the slider
    /// This will move the slider one step closer to its destination
    /// If the slider is idle, this function does nothing
    pub fn tick(&mut self) -> &mut Self {
        self.state = match &self.state {
            SliderState::IdleAt { .. } => self.state.clone(),
            SliderState::MoveTo { from, current, to } => {
                self.tick_(from.clone(), current.clone(), to.clone())
            }
        };
        self
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
