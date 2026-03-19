use crate::helpers::has_duplicate_values;
use crate::objects::slot::Slot;
use crate::traits::{HasSlots, Identifiable, Solvable};
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub struct Area {
    id: usize,
    slots: Vec<Rc<RefCell<Slot>>>,
}

impl Area {
    pub fn new(id: usize, slots: Vec<Rc<RefCell<Slot>>>) -> Self {
        if has_duplicate_values(&slots) {
            panic!("The area with id {id} has duplicates.");
        }
        Area { id, slots }
    }
}

impl Solvable for Area {
    fn is_solved(&self) -> bool {
        self.slots.iter().all(|slot| slot.borrow().is_solved())
    }
}

impl Identifiable for Area {
    fn id(&self) -> usize {
        self.id
    }
}

impl HasSlots for Area {
    fn slots(&self) -> &Vec<Rc<RefCell<Slot>>> {
        &self.slots
    }
}

impl Area {
    /// Get all solved slots in this area.
    pub fn solved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots
            .iter()
            .filter(|slot| slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    /// Get all unsolved slots in this area
    pub fn unsolved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots
            .iter()
            .filter(|slot| !slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    /// Get all solved values in this area
    pub fn solved_values(&self) -> Vec<u8> {
        self.slots
            .iter()
            .filter_map(|slot| slot.borrow().get_value())
            .collect()
    }

    /// Check if a value exists in this area
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

impl Debug for Area {
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
