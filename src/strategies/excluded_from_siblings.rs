
use crate::objects::line::Line;
use crate::traits::{HasCells, SimpleSudoku, Strategy};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use tracing::debug;

#[derive(Debug)]
pub struct ExcludedFromSiblingsInRow;

impl Display for ExcludedFromSiblingsInRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExcludedFromSiblingsInRow")
    }
}

impl<S: SimpleSudoku> Strategy<S> for ExcludedFromSiblingsInRow {
    fn run(&self, sudoku: &mut S) {
        debug!("{}: started.", self);
        let lines = sudoku.rows();
        apply_excluded_from_siblings_strategy::<S>(lines.to_vec(), self);
        debug!("{}: completed.", self);
    }
}

#[derive(Debug)]
pub struct ExcludedFromSiblingsInColumn;

impl Display for ExcludedFromSiblingsInColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExcludedFromSiblingsInColumn")
    }
}

impl<S: SimpleSudoku> Strategy<S> for ExcludedFromSiblingsInColumn {
    fn run(&self, sudoku: &mut S) {
        debug!("{}: started.", self);
        let lines = sudoku.columns();
        apply_excluded_from_siblings_strategy::<S>(lines.to_vec(), self);
        debug!("{}: completed.", self);
    }
}

fn apply_excluded_from_siblings_strategy<S: SimpleSudoku>(
    lines: Vec<Rc<RefCell<Line>>>,
    strategy: &impl Display,
) {
    loop {
        let mut progress_made = false;

        for rc_refcell_of_line in &lines {
            let ref_line = rc_refcell_of_line.borrow();

            for number in 1..=S::LENGTH as u8 {
                // Якщо число вже є в лінії, пропускаємо
                if ref_line
                    .cells()
                    .iter()
                    .any(|c| c.borrow().get_value() == Some(number))
                {
                    continue;
                }

                // Знаходимо клітинки, в яких можливе це число
                let possible_cells: Vec<_> = ref_line
                    .cells()
                    .iter()
                    .filter(|cell| {
                        let ref_cell = cell.borrow();
                        !ref_cell.is_solved() && ref_cell.variants().contains(&number)
                    })
                    .cloned()
                    .collect();

                // Якщо залишилась тільки одна можлива клітинка
                if possible_cells.len() == 1 {
                    let mut cell = possible_cells[0].borrow_mut();
                    if cell.set_value(Some(number)) {
                        progress_made = true;
                        debug!("{}: has solved the {}.", strategy, cell);
                    } else {
                        debug!("{}: already solved {}.", strategy, cell);
                    }
                }
            }
        }

        // Якщо прогресу немає → завершуємо
        if !progress_made {
            break;
        }
    }
}