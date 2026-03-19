use crate::objects::area::Area;
use crate::objects::line::Line;
use crate::objects::slot::Slot;
use crate::traits::{
    HasAreas, HasColumns, HasRows, HasSlots, SimpleSudoku, Solvable, SolveMetrics, SudokuConfig,
};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
pub struct Sudoku9x9 {
    slots: Vec<Rc<RefCell<Slot>>>,
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

impl HasSlots for Sudoku9x9 {
    fn slots(&self) -> &Vec<Rc<RefCell<Slot>>> {
        &self.slots
    }
}

impl Solvable for Sudoku9x9 {}

impl SolveMetrics for Sudoku9x9 {}

impl SimpleSudoku for Sudoku9x9 {
    fn create_sudoku(
        slots: Vec<Rc<RefCell<Slot>>>,
        rows: Vec<Rc<RefCell<Line>>>,
        columns: Vec<Rc<RefCell<Line>>>,
        areas: Vec<Rc<RefCell<Area>>>,
    ) -> Self {
        Self {
            slots,
            rows,
            columns,
            areas,
        }
    }
}

impl PartialEq<Self> for Sudoku9x9 {
    fn eq(&self, other: &Self) -> bool {
        self.slots() == other.slots()
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
                let slot = self.slots[idx].borrow();
                match slot.get_value() {
                    Some(val) => write!(f, "{} ", val)?,
                    None => write!(f, "* ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Sudoku9x9 {
    fn parse_single_line(s: &str) -> Vec<Vec<Option<u8>>> {
        // Accepts: "530070000600195000..." (0 OR . OR * for empty)
        s.chars()
            .filter_map(|ch| match ch {
                '1'..='9' => Some(Some(ch as u8 - b'0')),
                '0' | '.' | '*' => Some(None),
                _ => None, // skip separators, whitespace, newlines
            })
            .collect::<Vec<_>>()
            .chunks(Self::NUMBER_OF_COLS)
            .map(|row| row.to_vec())
            .collect()
    }

    fn parse_multi_line(s: &str) -> Vec<Vec<Option<u8>>> {
        // Accepts:
        // "5 3 . | . 7 . | . . .
        //  6 . . | 1 9 5 | . . .
        //  ------+-------+------"
        s.lines()
            .filter(|line| {
                // Skip separator lines like "------+-------+------"
                line.contains(|c: char| c.is_ascii_digit() || c == '.' || c == '*')
            })
            .map(|line| {
                line.chars()
                    .filter_map(|ch| match ch {
                        '1'..='9' => Some(Some(ch as u8 - b'0')),
                        '0' | '.' | '*' => Some(None),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum SudokuParseError {
    InvalidLength { expected: usize, got: usize },
    InvalidCharacter(char),
}

impl FromStr for Sudoku9x9 {
    type Err = SudokuParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = if s.contains('\n') {
            Self::parse_multi_line(s)
        } else {
            Self::parse_single_line(s)
        };

        let total: usize = data.iter().map(|r| r.len()).sum();
        if total != Self::total_number_of_slots() {
            return Err(SudokuParseError::InvalidLength {
                expected: Self::total_number_of_slots(),
                got: total,
            });
        }

        Ok(Self::new(data))
    }
}
