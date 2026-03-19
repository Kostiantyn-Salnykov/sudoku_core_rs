use crate::objects::line::Line;
use crate::traits::{SimpleSudoku, Strategy};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug)]
pub struct HiddenSingleInRowStrategy;

impl Display for HiddenSingleInRowStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HiddenSingleInRowStrategy")
    }
}

impl<S: SimpleSudoku> Strategy<S> for HiddenSingleInRowStrategy {
    fn run(&self, sudoku: &mut S) {
        info!("{}: started.", self);
        let lines = sudoku.rows();
        apply_hidden_single_strategy::<S>(lines.to_vec(), self);
        debug!("{}: completed.", self);
    }
}

#[derive(Debug)]
pub struct HiddenSingleInColumnStrategy;

impl Display for HiddenSingleInColumnStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HiddenSingleInColumnStrategy")
    }
}

impl<S: SimpleSudoku> Strategy<S> for HiddenSingleInColumnStrategy {
    fn run(&self, sudoku: &mut S) {
        info!("{}: started.", self);
        let lines = sudoku.columns();
        apply_hidden_single_strategy::<S>(lines.to_vec(), self);
        debug!("{}: completed.", self);
    }
}

fn apply_hidden_single_strategy<S: SimpleSudoku>(
    lines: Vec<Rc<RefCell<Line>>>,
    strategy: &impl Display,
) {
    loop {
        let mut progress_made = false;

        for rc_refcell_of_line in &lines {
            let ref_line = rc_refcell_of_line.borrow();

            for number in 1..=S::LENGTH as u8 {
                // If the value is already set, skip.
                if ref_line.has_value(number) {
                    continue;
                }

                // Possible slots that can have this value.
                let possible_slots = ref_line.slots_with_candidate(number);

                // Only one slot can have this value, so we can set it directly.
                if possible_slots.len() == 1 {
                    let mut slot = possible_slots[0].borrow_mut();
                    if slot.set_value(Some(number)) {
                        progress_made = true;
                        debug!("{}: has solved the {:#}.", strategy, slot);
                    } else {
                        debug!("{}: already solved {:#}.", strategy, slot);
                    }
                }
            }
        }

        // No progress was made, so we can stop the loop.
        if !progress_made {
            break;
        }
    }
}
