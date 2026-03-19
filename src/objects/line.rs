use crate::helpers::has_duplicate_values;
use crate::objects::slot::Slot;
use crate::traits::{HasSlots, Identifiable};
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub enum Alignment {
    Row,
    Column,
}

pub struct Line {
    id: usize,
    alignment: Alignment,
    slots: Vec<Rc<RefCell<Slot>>>,
}

impl Line {
    pub fn new(id: usize, alignment: Alignment, slots: Vec<Rc<RefCell<Slot>>>) -> Self {
        if has_duplicate_values(&slots) {
            panic!("The line with id {id} has duplicates.");
        }
        Line {
            id,
            alignment,
            slots,
        }
    }

    pub fn is_solved(&self) -> bool {
        self.slots.iter().all(|slot| slot.borrow().is_solved())
    }

    pub fn solved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots
            .iter()
            .filter(|slot| slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    pub fn unsolved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots
            .iter()
            .filter(|slot| !slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    /// Get all solved values in this line
    pub fn solved_values(&self) -> Vec<u8> {
        self.slots
            .iter()
            .filter_map(|slot| slot.borrow().get_value())
            .collect()
    }

    /// Check if a value exists in this line
    pub fn has_value(&self, value: u8) -> bool {
        self.slots
            .iter()
            .any(|slot| slot.borrow().get_value() == Some(value))
    }

    /// Get slots that can have this value (unsolved slots with this candidate)
    pub fn slots_with_candidate(&self, value: u8) -> Vec<Rc<RefCell<Slot>>> {
        self.slots
            .iter()
            .filter(|slot| {
                let slot_ref = slot.borrow();
                !slot_ref.is_solved() && slot_ref.has_candidate(value)
            })
            .cloned()
            .collect()
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line_type = match self.alignment {
            Alignment::Row => "Row",
            Alignment::Column => "Col",
        };
        // Get Slots values or "*".
        let slots_values: Vec<String> = self
            .slots
            .iter()
            .map(|slot| match slot.borrow().get_value() {
                Some(val) => val.to_string(),
                None => "*".to_string(),
            })
            .collect();
        write!(f, "{}-{} [{}]", line_type, self.id, slots_values.join(" "))
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let slots_ids: Vec<String> = self
            .slots()
            .iter()
            .map(|slot| slot.borrow().id().to_string())
            .collect();
        let ids_str = slots_ids.join(", ");
        write!(f, "{} [{}]", self.id(), ids_str)
    }
}

impl Identifiable for Line {
    fn id(&self) -> usize {
        self.id
    }
}

impl HasSlots for Line {
    fn slots(&self) -> &Vec<Rc<RefCell<Slot>>> {
        &self.slots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solved_true() {
        let mut slots = Vec::with_capacity(9);
        for val in 1..=9 {
            slots.push(Rc::new(RefCell::new(Slot::new(val, Some(val as u8)))))
        }
        let line = Line {
            id: 1,
            alignment: Alignment::Row,
            slots,
        };
        assert!(line.is_solved());
    }
}
