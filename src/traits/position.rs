use std::rc::Rc;
use crate::bottle::*;
pub trait Position {
    fn getBottles(&self) -> &Vec<Bottle>; // Holds the current arrangement of bottles.
    fn toString(&self, possiblePrevious: &Option<Rc<dyn Position>>) -> Result<String, String>;
}

