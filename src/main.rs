use sudoku_solver::objects::sudoku::Sudoku9x9;
use sudoku_solver::parsers::{load_csv, load_json};
use sudoku_solver::solver::Solver;
use sudoku_solver::strategies::{ExcludedFromSiblingsInRow, LastPossibleNumberStrategy};
use sudoku_solver::traits::SimpleSudoku;
use tracing::{debug, info};

fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let _data1 = load_json("fixtures/easy.json");
    // let data2 = load_csv("fixtures/easy.csv");
    let data2 = load_csv("fixtures/average.csv");

    for row in &data2 {
        debug!("{:?}", row);
    }

    let mut sudoku = Sudoku9x9::new(data2);
    let mut solver = Solver::new(&mut sudoku);
    solver.add_strategy(Box::new(ExcludedFromSiblingsInRow));
    solver.add_strategy(Box::new(LastPossibleNumberStrategy));
    solver.solve();
    // sudoku.display_cells_ids();
    // sudoku.display_column_ids();
    // sudoku.display_rows_ids();
    // sudoku.display_area_ids();

    let data_solved = load_csv("fixtures/average_solved.csv");
    let sudoku_solved = Sudoku9x9::new(data_solved);
    info!("Read from average_solved.csv:");
    info!("Solved sudoku: {sudoku_solved}");
    let solved_text = if sudoku == sudoku_solved { "Yes" } else { "No" };
    info!("Sudoku was solved properly: {}", solved_text);
}
