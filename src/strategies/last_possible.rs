use crate::objects::cell::Cell;
use crate::traits::{HasCells, Identifiable, SimpleSudoku, Strategy};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tracing::debug;

#[derive(Debug)]
pub struct LastPossibleNumberStrategy;

impl<S: SimpleSudoku> Strategy<S> for LastPossibleNumberStrategy {
    fn run(&self, sudoku: &mut S) {
        debug!("LastPossibleNumberStrategy: started.");

        loop {
            let mut progress_made = false;

            for cell_ref in sudoku.cells() {
                let cell_id = cell_ref.borrow().id();

                if cell_ref.borrow().is_solved() {
                    continue;
                }

                let mut items_to_remove: HashSet<u8> = HashSet::new();

                let mut collect_values = |cells: &[Rc<RefCell<Cell>>]| {
                    for other_cell in cells {
                        let other_cell_borrow = other_cell.borrow();
                        if other_cell_borrow.id() != cell_id {
                            if let Some(val) = other_cell_borrow.get_value() {
                                items_to_remove.insert(val);
                            }
                        }
                    }
                };

                if let Some(area) = cell_ref.borrow().area().upgrade() {
                    collect_values(area.borrow().cells());
                }

                if let Some(row) = cell_ref.borrow().row().upgrade() {
                    collect_values(row.borrow().cells());
                }

                if let Some(col) = cell_ref.borrow().column().upgrade() {
                    collect_values(col.borrow().cells());
                }

                {
                    let mut current_cell = cell_ref.borrow_mut();
                    let solved_before = current_cell.is_solved();
                    current_cell.exclude_values(items_to_remove);

                    if solved_before != current_cell.is_solved() {
                        progress_made = true;
                        debug!(
                            "LastPossibleNumberStrategy: has solved the {:#}.",
                            current_cell
                        );
                    }
                }
            }

            if !progress_made {
                break;
            }
        }

        debug!("LastPossibleNumberStrategy: completed.");
    }
}
