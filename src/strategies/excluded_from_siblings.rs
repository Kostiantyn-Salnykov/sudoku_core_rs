use crate::traits::{HasCells, SimpleSudoku, Strategy};

use tracing::debug;

#[derive(Debug)]
pub struct ExcludedFromSiblingsInRow;

impl<S: SimpleSudoku> Strategy<S> for ExcludedFromSiblingsInRow {
    fn run(&self, sudoku: &mut S) {
        debug!("ExcludedFromSiblingsInRow: started.");
        loop {
            let mut progress_made = false;
            for row_ref in sudoku.rows() {
                let row = row_ref.borrow();

                for number in 1..=9 {
                    let mut possible_cells = vec![];

                    for cell_ref in row.cells() {
                        let cell = cell_ref.borrow();
                        if cell.is_solved() {
                            continue;
                        }
                        if cell.variants().contains(&number) {
                            possible_cells.push(cell_ref.clone());
                        }
                    }

                    if possible_cells.len() == 1 {
                        let mut cell = possible_cells[0].borrow_mut();
                        cell.set_value(Some(number));
                        debug!("ExcludedFromSiblingsInRow: has solved the {:#}.", cell);
                        progress_made = true;
                    }
                }
            }

            // If no progress → done
            if !progress_made {
                break;
            }
        }

        debug!("ExcludedFromSiblingsInRow: completed.");
    }
}
