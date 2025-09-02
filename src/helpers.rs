use crate::objects::cell::Cell;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn has_duplicate_values(cells: &Vec<Rc<RefCell<Cell>>>) -> bool {
    let mut seen_values = HashSet::new();

    for cell_rc in cells {
        let cell = cell_rc.borrow();
        if let Some(value) = cell.get_value() {
            // Try to insert the value into the HashSet
            if !seen_values.insert(value) {
                // If insertion fails, a duplicate is found
                return true;
            }
        }
    }
    false
}
