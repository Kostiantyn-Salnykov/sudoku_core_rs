use sudoku_solver::objects::sudoku::Sudoku9x9;
use sudoku_solver::parsers::{load_csv, load_json};
use sudoku_solver::solver::Solver;
use sudoku_solver::strategies::{
    BacktrackingStrategy, ConstraintPropagationStrategy, HiddenSingleInColumnStrategy,
    HiddenSingleInRowStrategy,
};
use sudoku_solver::traits::SimpleSudoku;
use tracing::{debug, info};
use tracing_subscriber::fmt::format;

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .event_format(format().compact())
        // .event_format(format().json())
        // .with_max_level(Level::TRACE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("RUST_LOG"))
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber.");

    debug!("Tracing is set up.");
}

fn main() {
    dotenv::dotenv().ok();
    setup_tracing();

    let _data1 = load_json("fixtures/easy.json");
    // let data2 = load_csv("fixtures/easy.csv");
    // let data2 = load_csv("fixtures/average.csv");
    let data2 = load_csv("fixtures/hard_1.csv");
    // let data2 = load_csv("fixtures/hard_2.csv");

    for row in &data2 {
        debug!("{:?}", row);
    }

    let mut sudoku = Sudoku9x9::new(data2);
    sudoku.display_columns_ids();
    let mut solver = Solver::new(&mut sudoku);
    solver.add_strategy(Box::new(ConstraintPropagationStrategy));
    solver.add_strategy(Box::new(HiddenSingleInRowStrategy));
    solver.add_strategy(Box::new(HiddenSingleInColumnStrategy));
    solver.set_backtracking_strategy(Box::new(BacktrackingStrategy));
    solver.solve();
    // sudoku.display_cells_ids();
    // sudoku.display_column_ids();
    // sudoku.display_rows_ids();
    // sudoku.display_area_ids();

    // let data_solved = load_csv("fixtures/average_solved.csv");
    let data_solved = load_csv("fixtures/hard_1_solved.csv");
    // let data_solved = load_csv("fixtures/hard_2_solved.csv");
    let sudoku_solved = Sudoku9x9::new(data_solved);
    info!("Read from <solved>.csv:");
    info!("Solved sudoku: {sudoku_solved}");
    let solved_text = if sudoku == sudoku_solved { "Yes" } else { "No" };
    info!("Sudoku was solved properly: {}", solved_text);
}
