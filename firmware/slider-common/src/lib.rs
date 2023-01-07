//! Slider State Machine
//!
//! ```none
//!  Diagram:
//!   o
//!   │
//!  ┌▼──────────┐
//!  │ IdleAt(0) │
//!  └▲──────┬───┘
//!   │      │Goto(x)
//!   │ ┌────▼────────────┐
//!   │ │ MoveTo(from,to) │
//!   │ └──────┬──────────┘
//!   │Goto(0) │
//!  ┌┴────────▼──┐
//!  │ IdleAt(to) │
//!  └────────────┘
//! ```

#![no_std]

#[derive(Debug, PartialEq)]
pub struct Slider {
    pub state: SliderState,
}

/// SliderAction is an action that can be performed on the slider
#[derive(Debug, PartialEq)]
pub enum SliderAction {
    /// Stop the slider
    Stop,
    /// Go to position x
    Goto(u32),
}

/// Slider is a state machine that can be in one of two states:
/// IdleAt(x) or MoveTo(x, y)
#[derive(Debug, PartialEq, Clone)]
pub enum SliderState {
    /// Sldier is idle at position x
    IdleAt { position: u32 },
    /// Slider is moving from postion 'from' to position 'to'
    MoveTo { from: u32, current: u32, to: u32 },
}

impl Slider {
    /// Create a new slider
    /// The slider is initially idle at position 0
    pub fn new() -> Self {
        Slider {
            state: SliderState::IdleAt { position: 0 },
        }
    }

    /// Get the current position of the slider
    /// If the slider is moving, the position is the starting position
    pub fn position(&self) -> u32 {
        match self.state {
            SliderState::IdleAt { position } => position,
            SliderState::MoveTo { current, .. } => current,
        }
    }

    /// Get the target position of the slider
    /// If the slider is idle, the target position is the current position
    pub fn destination(&self) -> Option<u32> {
        match self.state {
            SliderState::IdleAt { .. } => None,
            SliderState::MoveTo { to, .. } => Some(to),
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
    pub fn tick(&mut self) -> &mut Self {
        self.state = match self.state {
            SliderState::IdleAt { .. } => self.state.clone(),
            SliderState::MoveTo { from, current, to } => {
                if current == to {
                    SliderState::IdleAt { position: to }
                } else if current < to {
                    SliderState::MoveTo {
                        from,
                        current: current + 1,
                        to,
                    }
                } else {
                    SliderState::MoveTo {
                        from,
                        current: current - 1,
                        to,
                    }
                }
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

    /// Test if the slider when stopped multiple time, it stops just once
    #[test]
    fn test_slider_stop() {
        let mut slider = Slider::new();
        slider.act(SliderAction::Goto(10));

        for _ in 0..10 {
            slider.tick();
        }

        slider.act(SliderAction::Stop);
        assert_eq!(slider.state, SliderState::IdleAt { position: 10 });
        slider.act(SliderAction::Stop);
        assert_eq!(slider.state, SliderState::IdleAt { position: 10 });
    }

    /// Test that the slider state machine works
    #[test]
    fn test_slider() {
        let mut slider = Slider::new();

        assert_eq!(slider.position(), 0);
        assert_eq!(slider.destination(), None);

        slider.act(SliderAction::Goto(10));

        slider.tick().tick().tick();

        assert_eq!(slider.position(), 3);
        assert_eq!(slider.destination(), Some(10));

        slider.act(SliderAction::Stop);
        assert_eq!(slider.position(), 3);
        assert_eq!(slider.destination(), None);
    }

    #[test]
    fn states_and_actions_work() {
        assert_eq!(Slider::new().state, SliderState::IdleAt { position: 0 });
        assert_eq!(
            Slider::new().act(SliderAction::Stop).state,
            SliderState::IdleAt { position: 0 }
        );
        assert_eq!(
            Slider::new().act(SliderAction::Goto(10)),
            &Slider {
                state: SliderState::MoveTo {
                    from: 0,
                    current: 0,
                    to: 10
                }
            }
        );

        let mut slider = Slider::new();
        slider
            .act(SliderAction::Goto(100))
            .act(SliderAction::Goto(20));

        for _ in 0..120 {
            slider.tick();
        }
        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt { position: 20 }
        );
    }

    #[test]
    fn position_works() {
        assert_eq!(Slider::new().position(), 0);
        assert_eq!(Slider::new().act(SliderAction::Goto(10)).position(), 0);
        let mut slider = Slider::new();

        slider
            .act(SliderAction::Goto(10))
            .act(SliderAction::Stop)
            .act(SliderAction::Goto(20));
        for _ in 0..100 {
            slider.tick();
        }

        assert_eq!(slider.act(SliderAction::Stop).position(), 20);
    }

    #[test]
    fn destination_works() {
        assert_eq!(Slider::new().destination(), None);
        assert_eq!(
            Slider::new()
                .act(SliderAction::Goto(10))
                .act(SliderAction::Stop)
                .destination(),
            None
        );
        assert_eq!(
            Slider::new()
                .act(SliderAction::Goto(100))
                .act(SliderAction::Goto(1000))
                .destination(),
            Some(1000)
        );
    }

    #[test]
    fn act_works() {
        let mut slider = Slider::new();
        slider
            .act(SliderAction::Goto(100))
            .act(SliderAction::Goto(5))
            .act(SliderAction::Goto(10));

        for _ in 0..115 {
            slider.tick();
        }

        assert_eq!(
            slider.act(SliderAction::Stop).state,
            SliderState::IdleAt { position: 10 }
        );
    }

    #[test]
    fn new_works() {
        assert_eq!(Slider::new().state, SliderState::IdleAt { position: 0 });
    }
}
