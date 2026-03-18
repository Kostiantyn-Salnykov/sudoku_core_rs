use crate::helpers::has_duplicate_values;
use crate::objects::cell::Cell;
use crate::traits::{HasCells, Identifiable, Solvable};
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub struct Area {
    id: usize,
    cells: Vec<Rc<RefCell<Cell>>>,
}

impl Area {
    pub fn new(id: usize, cells: Vec<Rc<RefCell<Cell>>>) -> Self {
        if has_duplicate_values(&cells) {
            panic!("The area with id {id} has duplicates.");
        }
        Area { id, cells }
    }
}

impl Solvable for Area {
    fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| cell.borrow().is_solved())
    }
}

impl Identifiable for Area {
    fn id(&self) -> usize {
        self.id
    }
}

impl HasCells for Area {
    fn cells(&self) -> &Vec<Rc<RefCell<Cell>>> {
        &self.cells
    }
}

impl Area {
    /// Get all solved cells in this area
    pub fn solved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(|cell| cell.borrow().is_solved())
            .cloned()
            .collect()
    }

    /// Get all unsolved cells in this area
    pub fn unsolved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(|cell| !cell.borrow().is_solved())
            .cloned()
            .collect()
    }

    /// Get all solved values in this area
    pub fn solved_values(&self) -> Vec<u8> {
        self.cells
            .iter()
            .filter_map(|cell| cell.borrow().get_value())
            .collect()
    }

    /// Check if a value exists in this area
    pub fn has_value(&self, value: u8) -> bool {
        self.cells
            .iter()
            .any(|cell| cell.borrow().get_value() == Some(value))
    }

    /// Get cells that can have this value (unsolved cells with this candidate)
    pub fn cells_with_candidate(&self, value: u8) -> Vec<Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(|cell| {
                let cell_ref = cell.borrow();
                !cell_ref.is_solved() && cell_ref.has_candidate(value)
            })
            .cloned()
            .collect()
    }
}

impl Debug for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cells_ids: Vec<String> = self
            .cells()
            .iter()
            .map(|cell| cell.borrow().id().to_string())
            .collect();
        let ids_str = cells_ids.join(", ");
        write!(f, "{} [{}]", self.id(), ids_str)
    }
}
