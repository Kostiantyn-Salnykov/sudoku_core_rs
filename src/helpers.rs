use crate::objects::slot::Slot;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn has_duplicate_values(slots: &Vec<Rc<RefCell<Slot>>>) -> bool {
    let mut seen_values = HashSet::new();

    for slot_rc in slots {
        let slot = slot_rc.borrow();
        if let Some(value) = slot.get_value() {
            // Try to insert the value into the HashSet
            if !seen_values.insert(value) {
                // If insertion fails, a duplicate is found
                return true;
            }
        }
    }
    false
}
