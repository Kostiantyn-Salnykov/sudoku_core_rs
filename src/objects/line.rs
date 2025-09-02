use crate::helpers::has_duplicate_values;
use crate::objects::cell::Cell;
use crate::traits::{HasCells, Identifiable};
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
    cells: Vec<Rc<RefCell<Cell>>>,
}

impl Line {
    pub fn new(id: usize, alignment: Alignment, cells: Vec<Rc<RefCell<Cell>>>) -> Self {
        if has_duplicate_values(&cells) {
            panic!("The line with id {id} has duplicates.");
        }
        Line {
            id,
            alignment,
            cells,
        }
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| cell.borrow().is_solved())
    }

    pub fn solved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(|cell| cell.borrow().is_solved())
            .cloned()
            .collect()
    }

    pub fn unsolved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(|cell| !cell.borrow().is_solved())
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
        // Get Cells values or "*".
        let cells_values: Vec<String> = self
            .cells
            .iter()
            .map(|cell| match cell.borrow().get_value() {
                Some(val) => val.to_string(),
                None => "*".to_string(),
            })
            .collect();
        write!(f, "{}-{} [{}]", line_type, self.id, cells_values.join(" "))
    }
}

impl Debug for Line {
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

impl Identifiable for Line {
    fn id(&self) -> usize {
        self.id
    }
}

impl HasCells for Line {
    fn cells(&self) -> &Vec<Rc<RefCell<Cell>>> {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solved_true() {
        let mut cells = Vec::with_capacity(9);
        let variants = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for val in 1..=9 {
            cells.push(Rc::new(RefCell::new(Cell::new(
                val,
                Some(val as u8),
                variants.clone(),
            ))))
        }
        let line = Line {
            id: 1,
            alignment: Alignment::Row,
            cells,
        };
        assert!(line.is_solved());
    }
}
