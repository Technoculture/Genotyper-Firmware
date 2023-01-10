/// A trait for a toggleable object.
pub trait Toggle {
    /// Toggle the state of the object.
    fn toggle(&mut self);
}

pub trait Switch {
    /// Check if the object is on.
    fn is_on(&self) -> bool;
    /// Check if the object is off.
    fn is_off(&self) -> bool {
        !self.is_on()
    }
    /// Switch the object on.
    fn on(&mut self);
    /// Switch the object off.
    fn off(&mut self);
}
