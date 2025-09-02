use crate::objects::area::Area;
use crate::objects::cell::Cell;
use crate::objects::line::{Alignment, Line};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::info;

pub trait Identifiable {
    fn id(&self) -> usize;
}

pub trait HasCells {
    fn cells(&self) -> &Vec<Rc<RefCell<Cell>>>;
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

pub trait Solvable: HasCells {
    fn is_solved(&self) -> bool {
        self.cells().iter().all(|cell| cell.borrow().is_solved())
    }
}

pub trait SolveMetrics: Solvable {
    fn unsolved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells()
            .iter()
            .filter(|cell| !cell.borrow().is_solved())
            .cloned()
            .collect()
    }

    fn solved_cells(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.cells()
            .iter()
            .filter(|cell| cell.borrow().is_solved())
            .cloned()
            .collect()
    }

    fn count_solved_cells(&self) -> usize {
        self.cells()
            .iter()
            .filter(|cell| cell.borrow().is_solved())
            .count()
    }

    fn count_solved_percentage(&self) -> f64 {
        self.count_solved_cells() as f64 * 100.0 / self.cells().len() as f64
    }
}

pub trait SudokuConfig {
    const LENGTH: usize;
    const NUMBER_OF_COLS: usize = Self::LENGTH;
    const NUMBER_OF_ROWS: usize = Self::LENGTH;
    const NUMBER_OF_COLS_IN_AREA: usize;
    const NUMBER_OF_ROWS_IN_AREA: usize;

    fn total_number_of_cells() -> usize {
        Self::NUMBER_OF_COLS * Self::NUMBER_OF_ROWS
    }

    fn number_cells_in_area() -> usize {
        Self::NUMBER_OF_COLS_IN_AREA * Self::NUMBER_OF_ROWS_IN_AREA
    }

    fn number_of_areas() -> usize {
        (Self::NUMBER_OF_ROWS / Self::NUMBER_OF_ROWS_IN_AREA)
            * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA)
    }
}

pub trait SimpleSudoku:
    SudokuConfig + SolveMetrics + HasCells + HasRows + HasColumns + HasAreas
{
    fn populate_cells(data: &[Vec<Option<u8>>]) -> Vec<Rc<RefCell<Cell>>> {
        let mut cells = Vec::with_capacity(Self::total_number_of_cells());
        for (row_num, _row) in data.iter().enumerate().take(Self::NUMBER_OF_ROWS) {
            for col_num in 0..Self::NUMBER_OF_COLS {
                // Select id for cell.
                let id = row_num * Self::NUMBER_OF_ROWS + col_num + 1;
                // Grab cell value from data.
                let value = data[row_num][col_num];
                // Populate variants for cell.
                let variants = match value {
                    Some(val) => vec![val],
                    None => (1..=Self::LENGTH as u8).collect(),
                };
                cells.push(Rc::new(RefCell::new(Cell::new(id, value, variants))));
            }
        }
        cells
    }

    fn populate_rows(cells: &[Rc<RefCell<Cell>>]) -> Vec<Rc<RefCell<Line>>> {
        let mut rows = Vec::with_capacity(Self::NUMBER_OF_ROWS);
        for row_idx in 1..=Self::NUMBER_OF_ROWS {
            let mut row_cells = Vec::with_capacity(Self::NUMBER_OF_ROWS);
            for col in 0..Self::NUMBER_OF_COLS {
                let idx = (row_idx - 1) * Self::NUMBER_OF_COLS + col;
                row_cells.push(cells[idx].clone());
            }
            rows.push(Rc::new(RefCell::new(Line::new(
                row_idx,
                Alignment::Row,
                row_cells,
            ))));
        }
        rows
    }

    fn populate_columns(cells: &[Rc<RefCell<Cell>>]) -> Vec<Rc<RefCell<Line>>> {
        let mut columns = Vec::with_capacity(Self::NUMBER_OF_COLS);
        for col_idx in 1..=Self::NUMBER_OF_COLS {
            let mut col_cells = Vec::with_capacity(Self::NUMBER_OF_COLS);
            for row in 0..Self::NUMBER_OF_ROWS {
                let idx = row * Self::NUMBER_OF_ROWS + (col_idx - 1);
                col_cells.push(cells[idx].clone());
            }
            columns.push(Rc::new(RefCell::new(Line::new(
                col_idx,
                Alignment::Column,
                col_cells,
            ))));
        }
        columns
    }

    fn populate_areas(cells: &[Rc<RefCell<Cell>>]) -> Vec<Rc<RefCell<Area>>> {
        let mut areas = Vec::with_capacity(Self::number_of_areas());
        for area_row in 0..Self::NUMBER_OF_ROWS_IN_AREA {
            for area_col in 0..Self::NUMBER_OF_COLS_IN_AREA {
                let mut area_cells = Vec::with_capacity(Self::number_cells_in_area());
                for row in 0..Self::NUMBER_OF_ROWS_IN_AREA {
                    for col in 0..Self::NUMBER_OF_COLS_IN_AREA {
                        let r_idx = area_row * Self::NUMBER_OF_ROWS_IN_AREA + row;
                        let c_idx = area_col * Self::NUMBER_OF_COLS_IN_AREA + col;
                        let idx = r_idx * Self::NUMBER_OF_COLS + c_idx;
                        area_cells.push(cells[idx].clone());
                    }
                }
                let area_index =
                    area_row * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA) + area_col + 1;
                areas.push(Rc::new(RefCell::new(Area::new(area_index, area_cells))));
            }
        }
        areas
    }

    fn new(data: Vec<Vec<Option<u8>>>) -> Self
    where
        Self: Sized,
    {
        let cells = Self::populate_cells(&data);
        let rows = Self::populate_rows(&cells);
        let columns = Self::populate_columns(&cells);
        let areas = Self::populate_areas(&cells);

        // Create back-references in cells
        for (row_num, row) in rows.iter().enumerate().take(Self::NUMBER_OF_ROWS) {
            for (col_num, column) in columns.iter().enumerate().take(Self::NUMBER_OF_COLS) {
                let cell_idx = row_num * Self::NUMBER_OF_COLS + col_num;
                let cell_rc = cells[cell_idx].clone();
                let mut cell = cell_rc.borrow_mut();

                cell.set_row(Rc::downgrade(row));
                cell.set_column(Rc::downgrade(column));

                let area_index = (row_num / Self::NUMBER_OF_ROWS_IN_AREA)
                    * (Self::NUMBER_OF_COLS / Self::NUMBER_OF_COLS_IN_AREA)
                    + (col_num / Self::NUMBER_OF_COLS_IN_AREA);
                cell.set_area(Rc::downgrade(&areas[area_index]));
            }
        }

        Self::create_sudoku(cells, rows, columns, areas)
    }

    fn create_sudoku(
        cells: Vec<Rc<RefCell<Cell>>>,
        rows: Vec<Rc<RefCell<Line>>>,
        columns: Vec<Rc<RefCell<Line>>>,
        areas: Vec<Rc<RefCell<Area>>>,
    ) -> Self;

    fn display_cells_ids(&self) {
        let cells_ids: Vec<String> = self
            .cells()
            .iter()
            .map(|cell| cell.borrow().id().to_string())
            .collect();
        for (idx, cell) in cells_ids.iter().enumerate() {
            if idx % 9 == 0 && idx != 0 {
                println!()
            }

            if cell.len() == 1 {
                print!("{cell}  ")
            } else {
                print!("{cell} ")
            }
        }
        println!()
    }

    fn display_rows_ids(&self) {
        for row in self.rows() {
            info!("{:#?}", row.borrow())
        }
    }

    fn display_columns_ids(&self) {
        for column in self.columns() {
            info!("{:#?}", column.borrow())
        }
    }

    fn display_areas_ids(&self) {
        for area in self.areas() {
            info!("{:#?}", area.borrow())
        }
    }

    fn display_rows(&self) {
        for row in self.rows() {
            info!("{}", row.borrow());
        }
    }

    fn display_columns(&self) {
        for column in self.columns() {
            info!("{}", column.borrow());
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
