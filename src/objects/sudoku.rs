use crate::objects::area::Area;
use crate::objects::cell::Cell;
use crate::objects::line::Line;
use crate::traits::{
    HasAreas, HasCells, HasColumns, HasRows, SimpleSudoku, Solvable, SolveMetrics, SudokuConfig,
};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sudoku9x9 {
    cells: Vec<Rc<RefCell<Cell>>>,
    rows: Vec<Rc<RefCell<Line>>>,
    columns: Vec<Rc<RefCell<Line>>>,
    areas: Vec<Rc<RefCell<Area>>>,
}

impl SudokuConfig for Sudoku9x9 {
    const LENGTH: usize = 9;
    const NUMBER_OF_COLS_IN_AREA: usize = 3;
    const NUMBER_OF_ROWS_IN_AREA: usize = 3;
}

impl HasAreas for Sudoku9x9 {
    fn areas(&self) -> &Vec<Rc<RefCell<Area>>> {
        &self.areas
    }
}

impl HasRows for Sudoku9x9 {
    fn rows(&self) -> &Vec<Rc<RefCell<Line>>> {
        &self.rows
    }
}

impl HasColumns for Sudoku9x9 {
    fn columns(&self) -> &Vec<Rc<RefCell<Line>>> {
        &self.columns
    }
}

impl HasCells for Sudoku9x9 {
    fn cells(&self) -> &Vec<Rc<RefCell<Cell>>> {
        &self.cells
    }
}

impl Solvable for Sudoku9x9 {}

impl SolveMetrics for Sudoku9x9 {}

impl SimpleSudoku for Sudoku9x9 {
    fn create_sudoku(
        cells: Vec<Rc<RefCell<Cell>>>,
        rows: Vec<Rc<RefCell<Line>>>,
        columns: Vec<Rc<RefCell<Line>>>,
        areas: Vec<Rc<RefCell<Area>>>,
    ) -> Self {
        Self {
            cells,
            rows,
            columns,
            areas,
        }
    }
}

impl PartialEq<Self> for Sudoku9x9 {
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}

impl Display for Sudoku9x9 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?; // Add a blank line.
        for row in 0..Self::NUMBER_OF_ROWS {
            // Add the horizontal separator for every third row.
            if row % Self::NUMBER_OF_ROWS_IN_AREA == 0 && row != 0 {
                writeln!(f, "------+-------+------")?;
            }

            for col in 0..Self::NUMBER_OF_COLS {
                // Add the vertical separator for every third column.
                if col % Self::NUMBER_OF_COLS_IN_AREA == 0 && col != 0 {
                    write!(f, "| ")?;
                }

                let idx = row * 9 + col;
                let cell = self.cells[idx].borrow();
                match cell.get_value() {
                    Some(val) => write!(f, "{} ", val)?,
                    None => write!(f, "* ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
