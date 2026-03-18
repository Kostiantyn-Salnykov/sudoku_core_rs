use crate::traits::{SimpleSudoku, Strategy};
use tracing::{debug, info};

#[derive(Debug)]
pub struct ConstraintPropagationStrategy;

impl<S: SimpleSudoku> Strategy<S> for ConstraintPropagationStrategy {
    fn run(&self, sudoku: &mut S) {
        info!("ConstraintPropagationStrategy: started.");

        loop {
            let mut progress_made = false;

            for cell_ref in sudoku.cells() {
                if cell_ref.borrow().is_solved() {
                    continue;
                }

                // Get all solved peer values using the helper method
                let items_to_remove = cell_ref.borrow().get_solved_peers_values();

                {
                    let mut current_cell = cell_ref.borrow_mut();
                    let solved_before = current_cell.is_solved();
                    current_cell.exclude_values(items_to_remove);

                    if solved_before != current_cell.is_solved() {
                        progress_made = true;
                        debug!(
                            "ConstraintPropagationStrategy: has solved the {:#}.",
                            current_cell
                        );
                    }
                }
            }

            if !progress_made {
                break;
            }
        }

        debug!("ConstraintPropagationStrategy: completed.");
    }
}
