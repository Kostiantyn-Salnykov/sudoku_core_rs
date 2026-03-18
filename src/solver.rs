use crate::strategies::{
    BacktrackingStrategy, ConstraintPropagationStrategy, HiddenSingleInColumnStrategy,
    HiddenSingleInRowStrategy,
};
use crate::traits::{SimpleSudoku, Strategy};
use chrono::Utc;
use std::fmt::Display;
use tracing::{info, warn};

pub struct Solver<'a, S: SimpleSudoku> {
    sudoku: &'a mut S,
    strategies: Vec<Box<dyn Strategy<S>>>,
    backtracking_strategy: Option<Box<dyn Strategy<S>>>,
}

impl<'a, S> Solver<'a, S>
where
    S: SimpleSudoku + Display,
{
    pub fn new(sudoku: &'a mut S) -> Self {
        Self {
            sudoku,
            strategies: vec![],
            backtracking_strategy: None,
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy<S>>) {
        self.strategies.push(strategy);
    }

    pub fn set_backtracking_strategy(&mut self, strategy: Box<dyn Strategy<S>>) {
        self.backtracking_strategy = Some(strategy);
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
            let before_solved_count = self.sudoku.count_solved_cells();

            for strategy in &self.strategies {
                let before_strategy = self.sudoku.count_solved_cells();
                strategy.run(self.sudoku);

                if self.sudoku.is_solved() {
                    break 'main;
                }

                if self.sudoku.count_solved_cells() > before_strategy {
                    continue 'main;
                }
            }

            if self.sudoku.count_solved_cells() == before_solved_count {
                // No progress made with regular strategies
                if !self.sudoku.is_solved()
                    && let Some(backtracking) = &self.backtracking_strategy
                {
                    warn!("No progress with regular strategies, attempting backtracking.");
                    backtracking.run(self.sudoku);
                }

                break 'main;
            }
        }
        let end_ts = Utc::now();
        let diff_millis = (end_ts - start_ts).num_milliseconds();
        let diff_micros = (end_ts - start_ts).num_microseconds().unwrap();
        info!("Time elapsed: {:?} us ({:?} ms).", diff_micros, diff_millis);
        info!(
            "Sudoku has {} of {} solved cells ({:.3}%).",
            self.sudoku.count_solved_cells(),
            self.sudoku.cells().len(),
            self.sudoku.count_solved_percentage()
        );
        self.print_is_solved();
        info!("{:#}", self.sudoku);
    }

    pub fn with_defaults(sudoku: &'a mut S) -> Self {
        let mut solver = Self::new(sudoku);
        solver.add_strategy(Box::new(ConstraintPropagationStrategy));
        solver.add_strategy(Box::new(HiddenSingleInRowStrategy));
        solver.add_strategy(Box::new(HiddenSingleInColumnStrategy));
        solver.set_backtracking_strategy(Box::new(BacktrackingStrategy));
        solver
    }

    pub fn solve_with_defaults(sudoku: &'a mut S) {
        let mut solver = Self::with_defaults(sudoku);
        solver.solve();
    }
}
