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

#[derive(Debug, PartialEq)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(pub u32, pub &'static Range);

impl Position {
    pub fn new(value: u32, range: &'static Range) -> Option<Self> {
        if value <= range.max && value >= range.min {
            return Some(Self(value, range));
        }
        None
    }
}

impl core::ops::Add for Position {
    type Output = Option<Self>;
    fn add(self, value: Self) -> Option<Self> {
        let new_value = self.0 + value.0;
        if new_value <= self.1.max && new_value >= self.1.min {
            return Some(Self(new_value, self.1));
        }
        None
    }
}

impl core::ops::Sub for Position {
    type Output = Option<Self>;
    fn sub(self, value: Self) -> Option<Self> {
        let new_value = self.0 - value.0;
        if new_value <= self.1.max && new_value >= self.1.min {
            return Some(Self(new_value, self.1));
        }
        None
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Slider {
    pub state: SliderState,
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
/// IdleAt(x) or MoveTo(x, y)
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
    pub fn new(range: &'static Range) -> Self {
        Slider {
            state: SliderState::IdleAt {
                position: Position(0, range),
            },
            range,
        }
    }

    /// Get the current position of the slider
    /// If the slider is moving, the position is the starting position
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
            SliderState::MoveTo { to, .. } => Some(*to),
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

    fn tick_(&mut self, from: Position, current: Position, to: Position) -> SliderState {
        let pos_delta = Position(1, self.range);
        if current == to {
            SliderState::IdleAt { position: to }
        } else if current < to {
            let current = (current + pos_delta).unwrap_or_else(|| current.clone());
            SliderState::MoveTo {
                from: from,
                current: current,
                to: to,
            }
        } else {
            let current = (current - pos_delta).unwrap_or_else(|| current.clone());
            SliderState::MoveTo {
                from: from,
                current: current,
                to: to,
            }
        }
    }

    /// Update the slider
    /// This will move the slider one step closer to its destination
    /// If the slider is idle, this function does nothing
    pub fn tick(&mut self) -> &mut Self {
        self.state = match &self.state {
            SliderState::IdleAt { .. } => self.state.clone(),
            SliderState::MoveTo { from, current, to } => self.tick_(*from, *current, *to),
        };
        self
    }
}

/// Tests
/// -----
#[cfg(test)]
mod tests {
    use super::*;

    /// Test if the slider when stopped multiple time, it stops just once
    #[test]
    fn test_slider_stop() {
        const RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);
        slider.act(SliderAction::Goto(Position(50, &RANGE)));

        for _ in 0..10 {
            slider.tick();
        }

        slider.act(SliderAction::Stop);
        assert_eq!(
            slider.state,
            SliderState::IdleAt {
                position: Position(10, &RANGE)
            }
        );
        slider.act(SliderAction::Stop);
        assert_eq!(
            slider.state,
            SliderState::IdleAt {
                position: Position(10, &RANGE)
            }
        );
    }

    /// Test that the slider state machine works
    #[test]
    fn test_slider() {
        const RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        assert_eq!(slider.position(), Position(0, &RANGE));
        assert_eq!(slider.destination(), None);

        slider.act(SliderAction::Goto(Position(50, &RANGE)));

        slider.tick().tick().tick();

        assert_eq!(slider.position(), Position(3, &RANGE));
        assert_eq!(slider.destination(), Some(Position(50, &RANGE)));

        slider.act(SliderAction::Stop);
        assert_eq!(slider.position(), Position(3, &RANGE));
        assert_eq!(slider.destination(), None);
    }

    #[test]
    fn states_and_actions_work() {
        const RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(
            Slider::new(&RANGE).state,
            SliderState::IdleAt {
                position: Position(0, &RANGE)
            }
        );
        assert_eq!(
            Slider::new(&RANGE).act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position(0, &RANGE)
            }
        );
        assert_eq!(
            Slider::new(&RANGE).act(SliderAction::Goto(Position(50, &RANGE))),
            &Slider {
                state: SliderState::MoveTo {
                    from: Position(0, &RANGE),
                    current: Position(0, &RANGE),
                    to: Position(50, &RANGE),
                },
                range: &RANGE,
            }
        );

        let mut slider = Slider::new(&RANGE);
        slider
            .act(SliderAction::Goto(Position(50, &RANGE)))
            .act(SliderAction::Goto(Position(100, &RANGE)));

        for _ in 0..150 {
            slider.tick();
        }
        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position(100, &RANGE)
            }
        );
    }

    #[test]
    fn position_works() {
        const RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        assert_eq!(Slider::new(&RANGE).position(), Position(0, &RANGE));
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position(100, &RANGE)))
                .position(),
            Position(0, &RANGE)
        );

        slider
            .act(SliderAction::Goto(Position(10, &RANGE)))
            .act(SliderAction::Stop)
            .act(SliderAction::Goto(Position(20, &RANGE)));
        for _ in 0..100 {
            slider.tick();
        }

        assert_eq!(
            slider.act(SliderAction::Stop).position(),
            Position(20, &RANGE)
        );
    }

    #[test]
    fn destination_works() {
        const RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(Slider::new(&RANGE).destination(), None);
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position(10, &RANGE)))
                .act(SliderAction::Stop)
                .destination(),
            None
        );
        assert_eq!(
            Slider::new(&RANGE)
                .act(SliderAction::Goto(Position(10, &RANGE)))
                .act(SliderAction::Goto(Position(20, &RANGE)))
                .destination(),
            Some(Position(20, &RANGE))
        );
    }

    #[test]
    fn act_works() {
        const RANGE: Range = Range { min: 0, max: 100 };
        let mut slider = Slider::new(&RANGE);

        slider
            .act(SliderAction::Goto(Position(10, &RANGE)))
            .act(SliderAction::Goto(Position(20, &RANGE)))
            .act(SliderAction::Goto(Position(30, &RANGE)));

        for _ in 0..60 {
            slider.tick();
        }

        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt {
                position: Position(30, &RANGE)
            }
        );
    }

    #[test]
    fn new_works() {
        const RANGE: Range = Range { min: 0, max: 100 };
        assert_eq!(
            Slider::new(&RANGE).state,
            SliderState::IdleAt {
                position: Position(0, &RANGE)
            }
        );
    }
}
