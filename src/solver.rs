use crate::traits::{SimpleSudoku, Strategy};
use chrono::Utc;
use std::fmt::Display;
use tracing::info;

pub struct Solver<'a, S: SimpleSudoku> {
    sudoku: &'a mut S,
    strategies: Vec<Box<dyn Strategy<S>>>,
}

impl<'a, S> Solver<'a, S>
where
    S: SimpleSudoku + Display,
{
    pub fn new(sudoku: &'a mut S) -> Self {
        Self {
            sudoku,
            strategies: vec![],
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy<S>>) {
        self.strategies.push(strategy);
    }

    fn print_is_solved(&self) {
        let solved_text = if self.sudoku.is_solved() { "Yes" } else { "No" };
        info!("Is solved now: {}", solved_text);
    }

    pub fn solve(&mut self) {
        info!(
            "Sudoku has {} of {} solved cells ({:.3}%).",
            self.sudoku.count_solved_cells(),
            self.sudoku.cells().len(),
            self.sudoku.count_solved_percentage()
        );
        self.print_is_solved();
        info!("{:#}", self.sudoku);
        info!("Solver started.");
        let start_ts = Utc::now();
        'main: loop {
            for strategy in &self.strategies {
                strategy.run(self.sudoku);
                if self.sudoku.is_solved() {
                    break 'main;
                }
            }
        }
        let end_ts = Utc::now();
        let diff_millis = (end_ts - start_ts).num_milliseconds();
        let diff_micros = (end_ts - start_ts).num_microseconds().unwrap();
        info!("Time elapsed: {:?} us ({:?} ms)", diff_micros, diff_millis);
        info!(
            "Sudoku has {} of {} solved cells ({:.3}%).",
            self.sudoku.count_solved_cells(),
            self.sudoku.cells().len(),
            self.sudoku.count_solved_percentage()
        );
        self.print_is_solved();
        info!("{:#}", self.sudoku);
    }
}
