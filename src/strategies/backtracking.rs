use crate::traits::{Identifiable, SimpleSudoku, Strategy, SudokuConfig};
use std::collections::HashSet;
use tracing::{debug, error, info, trace};

pub struct BacktrackingStrategy;

struct BacktrackState {
    values: Vec<Option<u8>>,
    candidates: Vec<HashSet<u8>>,
    peers: Vec<Vec<usize>>, // precomputed once
    total_slots: usize,
}

impl BacktrackState {
    fn from_sudoku<S: SimpleSudoku>(sudoku: &S) -> Self {
        let total_slots = S::total_number_of_slots();

        // Extract flat state from your Rc<RefCell<>> graph
        let mut values = vec![None; total_slots];
        let mut candidates = vec![HashSet::new(); total_slots];

        for slot in sudoku.slots() {
            let slot = slot.borrow();
            let idx = slot.id() - 1;
            values[idx] = slot.get_value();
            candidates[idx] = slot.variants();
        }

        let peers = Self::compute_peers::<S>();
        Self {
            values,
            candidates,
            peers,
            total_slots,
        }
    }

    fn compute_peers<S: SudokuConfig>() -> Vec<Vec<usize>> {
        let total_slots = S::total_number_of_slots();
        let num_rows = S::NUMBER_OF_ROWS;
        let num_cols = S::NUMBER_OF_COLS;
        let area_rows = S::NUMBER_OF_ROWS_IN_AREA;
        let area_cols = S::NUMBER_OF_COLS_IN_AREA;

        // For each slot, precompute its unique peers (row+col+area).
        (0..total_slots)
            .map(|i| {
                let row = i / num_cols;
                let col = i % num_cols;
                let area_row = (row / area_rows) * area_rows;
                let area_col = (col / area_cols) * area_cols;

                let mut peers = HashSet::new();

                // Same row
                for c in 0..num_cols {
                    peers.insert(row * num_cols + c);
                }

                // Same column
                for r in 0..num_rows {
                    peers.insert(r * num_cols + col);
                }

                // Same area
                for r in area_row..area_row + area_rows {
                    for c in area_col..area_col + area_cols {
                        peers.insert(r * num_cols + c);
                    }
                }

                peers.remove(&i);
                peers.into_iter().collect()
            })
            .collect()
    }

    fn apply_solution<S: SimpleSudoku>(&self, sudoku: &mut S) {
        // Write a solution back into your Rc<RefCell<>> graph
        for slot in sudoku.slots() {
            let mut slot = slot.borrow_mut();
            let idx = slot.id() - 1;
            if let Some(val) = self.values[idx] {
                slot.set_value(Some(val));
            }
        }
    }
}

fn solve(state: &mut BacktrackState, depth: usize) -> bool {
    // Find the unsolved slot with the fewest candidates (MRV heuristic).
    let Some(idx) = (0..state.total_slots)
        .filter(|&i| state.values[i].is_none())
        .min_by_key(|&i| state.candidates[i].len())
    else {
        debug!("BacktrackingStrategy: Solution found at depth {}.", depth);
        return true;
    };

    // If a slot has no candidates, this branch is invalid
    if state.candidates[idx].is_empty() {
        debug!(
            "BacktrackingStrategy: No candidates for slot {} at depth {}.",
            idx + 1,
            depth
        );
        return false;
    }

    let candidates: Vec<u8> = state.candidates[idx].iter().copied().collect();
    trace!(
        "BacktrackingStrategy: Trying slot {} (depth {}) with {} candidates: {:?}.",
        idx + 1,
        depth,
        candidates.len(),
        candidates
    );

    for candidate in candidates {
        // Save the current slot's candidates and all affected peers
        let saved_current = state.candidates[idx].clone();
        let saved_peers: Vec<(usize, HashSet<u8>)> = state.peers[idx]
            .iter()
            .map(|&p| (p, state.candidates[p].clone()))
            .collect();

        // Apply the candidate
        state.values[idx] = Some(candidate);
        state.candidates[idx] = HashSet::from([candidate]);

        // Propagate constraint: remove this candidate from all peers
        let mut valid = true;
        for &peer in &state.peers[idx] {
            if state.values[peer].is_none() {
                state.candidates[peer].remove(&candidate);
                // Check if peer still has at least one candidate
                if state.candidates[peer].is_empty() {
                    valid = false;
                    debug!(
                        "BacktrackingStrategy: Candidate {} for slot {} conflicts with peer {}.",
                        candidate,
                        idx + 1,
                        peer + 1
                    );
                    break;
                }
            }
        }

        // Recursively solve if constraints are satisfied
        if valid && solve(state, depth + 1) {
            return true;
        }

        debug!(
            "BacktrackingStrategy: Candidate {} for slot {} failed, backtracking.",
            candidate,
            idx + 1
        );

        // Undo: restore the current slot and all peers.
        state.values[idx] = None;
        state.candidates[idx] = saved_current;
        for (peer, saved_candidates) in saved_peers {
            state.candidates[peer] = saved_candidates;
        }
    }

    false
}

impl<S: SimpleSudoku> Strategy<S> for BacktrackingStrategy {
    fn run(&self, sudoku: &mut S) {
        info!("BacktrackingStrategy: started.");
        let unsolved_count = sudoku
            .slots()
            .iter()
            .filter(|c| c.borrow().get_value().is_none())
            .count();
        debug!(
            "BacktrackingStrategy: {} unsolved slots remaining.",
            unsolved_count
        );

        let mut state = BacktrackState::from_sudoku(sudoku);
        if solve(&mut state, 0) {
            state.apply_solution(sudoku);
            debug!("BacktrackingStrategy: Successfully solved the puzzle.");
        } else {
            error!("BacktrackingStrategy: No solution found.");
        }
    }
}
