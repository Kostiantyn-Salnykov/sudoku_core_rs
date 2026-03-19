use crate::objects::area::Area;
use crate::objects::line::{Alignment, Line};
use crate::objects::slot::Slot;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::trace;

pub trait Identifiable {
    fn id(&self) -> usize;
}

pub trait HasSlots {
    fn slots(&self) -> &Vec<Rc<RefCell<Slot>>>;
}

pub trait HasRows {
    fn rows(&self) -> &Vec<Rc<RefCell<Line>>>;
}

pub trait HasColumns {
    fn columns(&self) -> &Vec<Rc<RefCell<Line>>>;
}

pub trait HasAreas {
    fn areas(&self) -> &Vec<Rc<RefCell<Area>>>;
}

pub trait Solvable: HasSlots {
    fn is_solved(&self) -> bool {
        self.slots().iter().all(|slot| slot.borrow().is_solved())
    }
}

pub trait SolveMetrics: Solvable {
    fn unsolved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots()
            .iter()
            .filter(|slot| !slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    fn solved_slots(&self) -> Vec<Rc<RefCell<Slot>>> {
        self.slots()
            .iter()
            .filter(|slot| slot.borrow().is_solved())
            .cloned()
            .collect()
    }

    fn count_solved_slots(&self) -> usize {
        self.slots()
            .iter()
            .filter(|slot| slot.borrow().is_solved())
            .count()
    }

    fn count_solved_percentage(&self) -> f64 {
        self.count_solved_slots() as f64 * 100.0 / self.slots().len() as f64
    }
}

pub trait SudokuConfig {
    const LENGTH: usize;
    const NUMBER_OF_COLS: usize = Self::LENGTH;
    const NUMBER_OF_ROWS: usize = Self::LENGTH;
    const NUMBER_OF_COLS_IN_AREA: usize;
    const NUMBER_OF_ROWS_IN_AREA: usize;

    fn total_number_of_slots() -> usize {
        Self::NUMBER_OF_COLS * Self::NUMBER_OF_ROWS
    }

    fn number_of_slots_in_area() -> usize {
        Self::NUMBER_OF_COLS_IN_AREA * Self::NUMBER_OF_ROWS_IN_AREA
    }

    fn number_of_areas() -> usize {
        (Self::NUMBER_OF_ROWS / Self::NUMBER_OF_ROWS_IN_AREA)
            * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA)
    }
}

pub trait SimpleSudoku:
    SudokuConfig + SolveMetrics + HasSlots + HasRows + HasColumns + HasAreas
{
    fn populate_slots(data: &[Vec<Option<u8>>]) -> Vec<Rc<RefCell<Slot>>> {
        let mut slots = Vec::with_capacity(Self::total_number_of_slots());
        for (row_num, _row) in data.iter().enumerate().take(Self::NUMBER_OF_ROWS) {
            for (col_num, _) in data.iter().enumerate().take(Self::NUMBER_OF_COLS) {
                // Select id for a slot.
                let id = row_num * Self::NUMBER_OF_ROWS + col_num + 1;
                // Grab slot value from data.
                let value = data[row_num][col_num];
                slots.push(Rc::new(RefCell::new(Slot::new(id, value))));
            }
        }
        slots
    }

    fn populate_rows(slots: &[Rc<RefCell<Slot>>]) -> Vec<Rc<RefCell<Line>>> {
        let mut rows = Vec::with_capacity(Self::NUMBER_OF_ROWS);
        for row_idx in 1..=Self::NUMBER_OF_ROWS {
            let mut row_slots = Vec::with_capacity(Self::NUMBER_OF_ROWS);
            for col in 0..Self::NUMBER_OF_COLS {
                let idx = (row_idx - 1) * Self::NUMBER_OF_COLS + col;
                row_slots.push(slots[idx].clone());
            }
            rows.push(Rc::new(RefCell::new(Line::new(
                row_idx,
                Alignment::Row,
                row_slots,
            ))));
        }
        rows
    }

    fn populate_columns(slots: &[Rc<RefCell<Slot>>]) -> Vec<Rc<RefCell<Line>>> {
        let mut columns = Vec::with_capacity(Self::NUMBER_OF_COLS);
        for col_idx in 1..=Self::NUMBER_OF_COLS {
            let mut col_slots = Vec::with_capacity(Self::NUMBER_OF_COLS);
            for row in 0..Self::NUMBER_OF_ROWS {
                let idx = row * Self::NUMBER_OF_COLS + (col_idx - 1);
                col_slots.push(slots[idx].clone());
            }
            columns.push(Rc::new(RefCell::new(Line::new(
                col_idx,
                Alignment::Column,
                col_slots,
            ))));
        }
        columns
    }

    fn populate_areas(slots: &[Rc<RefCell<Slot>>]) -> Vec<Rc<RefCell<Area>>> {
        let mut areas = Vec::with_capacity(Self::number_of_areas());
        for area_row in 0..Self::NUMBER_OF_ROWS_IN_AREA {
            for area_col in 0..Self::NUMBER_OF_COLS_IN_AREA {
                let mut area_slots = Vec::with_capacity(Self::number_of_slots_in_area());
                for row in 0..Self::NUMBER_OF_ROWS_IN_AREA {
                    for col in 0..Self::NUMBER_OF_COLS_IN_AREA {
                        let r_idx = area_row * Self::NUMBER_OF_ROWS_IN_AREA + row;
                        let c_idx = area_col * Self::NUMBER_OF_COLS_IN_AREA + col;
                        let idx = r_idx * Self::NUMBER_OF_COLS + c_idx;
                        area_slots.push(slots[idx].clone());
                    }
                }
                let area_index =
                    area_row * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA) + area_col + 1;
                areas.push(Rc::new(RefCell::new(Area::new(area_index, area_slots))));
            }
        }
        areas
    }

    fn new(data: Vec<Vec<Option<u8>>>) -> Self
    where
        Self: Sized,
    {
        let slots = Self::populate_slots(&data);
        let rows = Self::populate_rows(&slots);
        let columns = Self::populate_columns(&slots);
        let areas = Self::populate_areas(&slots);

        // Create back-references in slots.
        for (row_num, row) in rows.iter().enumerate().take(Self::NUMBER_OF_ROWS) {
            for (col_num, column) in columns.iter().enumerate().take(Self::NUMBER_OF_COLS) {
                let slot_idx = row_num * Self::NUMBER_OF_COLS + col_num;
                let slot_rc = slots[slot_idx].clone();
                let mut slot = slot_rc.borrow_mut();

                slot.set_row(Rc::downgrade(row));
                slot.set_column(Rc::downgrade(column));

                let area_index = (row_num / Self::NUMBER_OF_ROWS_IN_AREA)
                    * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA)
                    + (col_num / Self::NUMBER_OF_COLS_IN_AREA);
                slot.set_area(Rc::downgrade(&areas[area_index]));
            }
        }

        Self::create_sudoku(slots, rows, columns, areas)
    }

    fn create_sudoku(
        slots: Vec<Rc<RefCell<Slot>>>,
        rows: Vec<Rc<RefCell<Line>>>,
        columns: Vec<Rc<RefCell<Line>>>,
        areas: Vec<Rc<RefCell<Area>>>,
    ) -> Self;

    fn display_slots_ids(&self) {
        let slots_ids: Vec<String> = self
            .slots()
            .iter()
            .map(|slot| slot.borrow().id().to_string())
            .collect();
        for (idx, slot) in slots_ids.iter().enumerate() {
            if idx % Self::NUMBER_OF_COLS == 0 && idx != 0 {
                println!()
            }

            if slot.len() == 1 {
                print!("{slot}  ")
            } else {
                print!("{slot} ")
            }
        }
        println!()
    }

    fn display_rows_ids(&self) {
        for row in self.rows() {
            trace!("{:#?}", row.borrow())
        }
    }

    fn display_columns_ids(&self) {
        for column in self.columns() {
            trace!("{:#?}", column.borrow())
        }
    }

    fn display_areas_ids(&self) {
        for area in self.areas() {
            trace!("{:#?}", area.borrow())
        }
    }

    fn display_rows(&self) {
        for row in self.rows() {
            trace!("{}", row.borrow());
        }
    }

    fn display_columns(&self) {
        for column in self.columns() {
            trace!("{}", column.borrow());
        }
    }
}

pub trait Strategy<S: SimpleSudoku> {
    fn run(&self, sudoku: &mut S);
}

pub trait Solver<S: SimpleSudoku> {
    fn new(sudoku: &mut S) -> Self;

    fn solve(&mut self);

    fn add_strategy(&mut self, strategy: Box<dyn Strategy<S>>);
}
