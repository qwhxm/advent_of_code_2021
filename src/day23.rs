//! <https://adventofcode.com/2021/day/23>
//!
//! It turns out A* wasn't necessary, Dijkstra would have been enough (the current implementation
//! runs only ~1.5 s slower without any heuristic). But I misunderstood the puzzle specification:
//!
//! >Amphipods will never move from the hallway into a room unless (...)
//!
//! Why does it say "from the hallway into a room"?! Why not just "into a room"? I originally
//! interpreted this as saying that an amphipod can move from a room into a "wrong" room, if it is
//! done without stopping in the hallway (which meant that the state space to search was larger).

use lazy_static::lazy_static;
use simple_grid::{Grid, GridIndex};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, VecDeque};

const INPUT_PART_1: [&str; 5] = [
    "#############",
    "#...........#",
    "###B#B#D#D###",
    "  #C#C#A#A#",
    "  #########",
];

const INPUT_PART_2: [&str; 7] = [
    "#############",
    "#...........#",
    "###B#B#D#D###",
    "  #D#C#B#A#",
    "  #D#B#A#C#",
    "  #C#C#A#A#",
    "  #########",
];

const TARGET_STATE_PART_1: [&str; 5] = [
    "#############",
    "#...........#",
    "###A#B#C#D###",
    "  #A#B#C#D#",
    "  #########",
];

const TARGET_STATE_PART_2: [&str; 7] = [
    "#############",
    "#...........#",
    "###A#B#C#D###",
    "  #A#B#C#D#",
    "  #A#B#C#D#",
    "  #A#B#C#D#",
    "  #########",
];

const OPEN_SPACE: char = '.';
const WALL: char = '#';

lazy_static! {
    static ref MOVE_COST_PER_AMPHIPOD_TYPE: HashMap<char, u32> =
        HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000),]);
    static ref TARGET_COLUMN_PER_AMPHIPOD_TYPE: HashMap<char, usize> =
        HashMap::from([('A', 3), ('B', 5), ('C', 7), ('D', 9),]);
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct BurrowState {
    /// State of the burrow, in the style of the input diagram. Each cell is one of:
    /// * [WALL],
    /// * [OPEN_SPACE],
    /// * an amphipod ([is_amphipod]).
    grid: Grid<char>,
}

fn is_amphipod(char: char) -> bool {
    MOVE_COST_PER_AMPHIPOD_TYPE.contains_key(&char)
}

/// Determines whether the given index (must be a walkable space) is in the hallway.
fn is_in_hallway(index: GridIndex) -> bool {
    index.row() == 1
}

/// Determines whether the given index (must be a walkable space) is in a room.
fn is_in_room(index: GridIndex) -> bool {
    !is_in_hallway(index)
}

/// Determines whether the given index is immediately outside a room.
fn is_room_entrance(index: GridIndex) -> bool {
    [
        GridIndex::new(3, 1),
        GridIndex::new(5, 1),
        GridIndex::new(7, 1),
        GridIndex::new(9, 1),
    ]
    .contains(&index)
}

/// Determines whether the given indices (both must be walkable spaces) are in the same room.
fn are_in_the_same_room(i1: GridIndex, i2: GridIndex) -> bool {
    is_in_room(i1) && is_in_room(i2) && i1.column() == i2.column()
}

impl BurrowState {
    fn from_rows(rows: &[&str]) -> BurrowState {
        let width = rows[0].len();
        let height = rows.len();
        let data = rows
            .iter()
            .map(|r| format!("{:<width$}", r, width = width))
            .map(|r| r.replace(' ', "#"))
            .collect::<Vec<String>>()
            .join("")
            .chars()
            .collect();
        BurrowState {
            grid: Grid::new(width, height, data),
        }
    }

    fn adjacent_open_spaces(&self, index: GridIndex) -> impl Iterator<Item = GridIndex> + '_ {
        let (c, r) = (index.column(), index.row());
        [
            GridIndex::new(c, r - 1),
            GridIndex::new(c, r + 1),
            GridIndex::new(c - 1, r),
            GridIndex::new(c + 1, r),
        ]
        .into_iter()
        .filter(|&i| self.grid[i] == OPEN_SPACE)
    }

    fn correct_room(&self, amphipod_type: char) -> Vec<GridIndex> {
        let column = TARGET_COLUMN_PER_AMPHIPOD_TYPE[&amphipod_type];
        let mut indices_in_room = Vec::new();
        for r in 0..self.grid.height() {
            let index = GridIndex::new(column, r);
            if self.grid[index] != WALL && is_in_room(index) {
                indices_in_room.push(index);
            }
        }
        indices_in_room
    }

    /// Determines whether it's within the rules for the given amphipod to move to the given
    /// destination.
    fn is_valid_move(&self, amphipod: GridIndex, dest_index: GridIndex) -> bool {
        if is_room_entrance(dest_index) {
            // "Amphipods will never stop on the space immediately outside any room."
            return false;
        }

        if is_in_room(dest_index) {
            let amphipod_type = self.grid[amphipod];
            let correct_room = self.correct_room(amphipod_type);
            if !correct_room.contains(&dest_index) {
                // "Amphipods will never move (...) into a room unless that room is their
                // destination room"...
                return false;
            }
            let mut room_occupants = correct_room
                .iter()
                .map(|&i| self.grid[i])
                .filter(|&c| c != OPEN_SPACE);
            if !room_occupants.all(|c| c == amphipod_type) {
                // ..."and that room contains no amphipods which do not also have that room as their
                // own destination."
                return false;
            }
        }

        if is_in_hallway(amphipod) && !is_in_room(dest_index) {
            // "Once an amphipod stops moving in the hallway, it will stay in that spot until it
            // can move into a room."
            return false;
        }

        true
    }

    /// Determines whether the specified move is worth considering when solving the amphipod
    /// organizing puzzle.
    fn is_pointless_move(&self, amphipod: GridIndex, dest_index: GridIndex) -> bool {
        let below_dest_index = GridIndex::new(dest_index.column(), dest_index.row() + 1);
        if is_in_room(dest_index) && self.grid[below_dest_index] == OPEN_SPACE {
            // It's pointless to leave an open space below when moving into a room.
            return true;
        }

        if are_in_the_same_room(amphipod, dest_index) {
            // It's pointless to move up or down within the same room.
            return true;
        }

        false
    }

    /// Returns all destinations, with their distances, for the given amphipod to move to that are
    /// worth considering when solving the amphipod organizing puzzle.
    fn moves_to_consider(
        &self,
        amphipod: GridIndex,
    ) -> impl Iterator<Item = (GridIndex, u32)> + '_ {
        /// Returns all nodes, with their distances, reachable from the given node (excluding the
        /// given node itself), using breadth-first search with the given successor function.
        fn bfs_reachable_nodes_with_distances<F, I>(
            initial_node: GridIndex,
            successors: F,
        ) -> HashMap<GridIndex, u32>
        where
            F: Fn(GridIndex) -> I,
            I: IntoIterator<Item = GridIndex>,
        {
            let mut distances = HashMap::from([(initial_node, 0)]);
            let mut nodes_to_be_visited = VecDeque::from([initial_node]);

            while let Some(node) = nodes_to_be_visited.pop_front() {
                let distance = distances[&node];
                for successor in successors(node) {
                    if let Entry::Vacant(entry) = distances.entry(successor) {
                        entry.insert(distance + 1);
                        nodes_to_be_visited.push_back(successor);
                    }
                }
            }

            distances.remove(&initial_node);
            distances
        }

        assert!(is_amphipod(self.grid[amphipod]));

        bfs_reachable_nodes_with_distances(amphipod, |i| self.adjacent_open_spaces(i))
            .into_iter()
            .filter(move |&(i, _)| self.is_valid_move(amphipod, i))
            .filter(move |&(i, _)| !self.is_pointless_move(amphipod, i))
    }

    /// Returns successors of this state in the state space graph of the amphipod organizing puzzle.
    ///
    /// (So edge weights are energy costs.)
    fn successors(&self) -> Vec<(BurrowState, u32)> {
        let mut successors = Vec::new();
        for r in 0..self.grid.height() {
            for c in 0..self.grid.width() {
                let index = GridIndex::new(c, r);
                if is_amphipod(self.grid[index]) {
                    for (dest_index, distance) in self.moves_to_consider(index) {
                        let mut successor = self.clone();
                        successor.grid.swap_cells(index, dest_index);

                        let cost = distance * MOVE_COST_PER_AMPHIPOD_TYPE[&self.grid[index]];

                        successors.push((successor, cost));
                    }
                }
            }
        }
        successors
    }

    /// A consistent heuristic for the A* algorithm in the state space graph of the amphipod
    /// organizing puzzle.
    ///
    /// Returns an estimation of the energy cost needed to get from this state to the target state.
    fn a_star_heuristic(&self) -> u32 {
        let mut total_heuristic_cost_to_organize = 0;
        let mut num_unorganized_amphipods_per_type =
            HashMap::from([('A', 0), ('B', 0), ('C', 0), ('D', 0)]);
        for r in 0..self.grid.height() {
            for c in 0..self.grid.width() {
                let index = GridIndex::new(c, r);
                if is_amphipod(self.grid[index]) {
                    let amphipod_type = self.grid[index];

                    if !self.correct_room(amphipod_type).contains(&index) {
                        // For amphipods not in the correct room, add to
                        // `total_heuristic_cost_to_organize` the energy cost needed to get to
                        // entrance of the correct room, in the ideal scenario (no obstacles).
                        let vertical_distance = c - 1;
                        let horizontal_distance =
                            (TARGET_COLUMN_PER_AMPHIPOD_TYPE[&amphipod_type] as i32 - c as i32)
                                .abs();
                        let heuristic_num_moves_to_correct_room_entrance =
                            vertical_distance as u32 + horizontal_distance as u32;

                        total_heuristic_cost_to_organize +=
                            heuristic_num_moves_to_correct_room_entrance
                                * MOVE_COST_PER_AMPHIPOD_TYPE[&amphipod_type];
                        *num_unorganized_amphipods_per_type
                            .get_mut(&amphipod_type)
                            .unwrap() += 1;
                    }

                    if self.correct_room(amphipod_type).contains(&index)
                        && self
                            .correct_room(amphipod_type)
                            .into_iter()
                            .filter(|i| i.row() > index.row())
                            .map(|i| self.grid[i])
                            .any(|c| is_amphipod(c) && c != amphipod_type)
                    {
                        // Amphipods that are in the correct room, but are blocking some amphipods
                        // of different type below them, will have to let those blocked amphipods
                        // out. So add to `total_heuristic_cost_to_organize` the energy cost needed
                        // to get out of the room, sidestep one space, and get back to the entrance
                        // (in the ideal scenario, so disregarding any obstacles).
                        let heuristic_num_moves_to_let_occupants_below_out = (c - 1) + 1 + 1;

                        total_heuristic_cost_to_organize +=
                            heuristic_num_moves_to_let_occupants_below_out as u32
                                * MOVE_COST_PER_AMPHIPOD_TYPE[&amphipod_type];
                        *num_unorganized_amphipods_per_type
                            .get_mut(&amphipod_type)
                            .unwrap() += 1;
                    }
                }
            }
        }

        // `total_heuristic_cost_to_organize` now includes, for each amphipod that is or will be
        // outside of its correct room, the cost of getting to entrance of the correct room.
        // Now, add also the costs of actually slotting into the rooms.
        for (amphipod_type, num_amphipods) in num_unorganized_amphipods_per_type {
            total_heuristic_cost_to_organize += ((num_amphipods * (num_amphipods + 1)) / 2)
                * MOVE_COST_PER_AMPHIPOD_TYPE[&amphipod_type];
        }

        total_heuristic_cost_to_organize
    }
}

fn least_energy_to_organize(initial_state: &BurrowState) -> u32 {
    #[derive(Eq, PartialEq)]
    struct FrontierItem {
        node: BurrowState,
        f_score: u32,
    }

    impl Ord for FrontierItem {
        fn cmp(&self, other: &Self) -> Ordering {
            // The comparison is reversed so we can use [std::collections::BinaryHeap] as a
            // min-heap.
            // XXX This is inconsistent with [PartialEq], the [BurrowState]s should also be
            //     compared. But it works even like this ¯\_(ツ)_/¯.
            self.f_score.cmp(&other.f_score).reverse()
        }
    }

    impl PartialOrd for FrontierItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn a_star(
        start_node: &BurrowState,
        dest_node: &BurrowState,
        successors: impl Fn(&BurrowState) -> Vec<(BurrowState, u32)>,
        h_score: impl Fn(&BurrowState) -> u32,
    ) -> u32 {
        let mut final_distances = HashMap::new();
        let mut tentative_distances = HashMap::from([(start_node.clone(), 0)]);
        let mut frontier = BinaryHeap::from([FrontierItem {
            node: start_node.clone(),
            f_score: 0,
        }]);

        loop {
            match frontier.pop() {
                None => panic!("Destination node not reachable"),
                Some(FrontierItem { node, .. }) => {
                    // Tentative distance to the node with the lowest f-score is now the final
                    // distance.
                    // XXX Can't remove from `tentative_distances` here because there may be
                    //     multiple entries for the same node in `frontier`.
                    let distance = *tentative_distances.get(&node).unwrap();
                    final_distances.insert(node.clone(), distance);

                    if node == *dest_node {
                        break distance;
                    }

                    // Try to improve tentative distances (and consequently also f-scores) of
                    // successors of the newly finalized node (only those successors whose distance
                    // is not finalized yet).
                    for (succ, edge_len) in successors(&node)
                        .into_iter()
                        .filter(|(succ, _)| !final_distances.contains_key(succ))
                    {
                        let distance_through_current_node = distance + edge_len;
                        if distance_through_current_node
                            < *tentative_distances.get(&succ).unwrap_or(&u32::MAX)
                        {
                            tentative_distances.insert(succ.clone(), distance_through_current_node);
                            frontier.push(FrontierItem {
                                node: succ.clone(),
                                f_score: distance_through_current_node + h_score(&succ),
                            });
                        }
                    }
                }
            }
        }
    }

    let target_state = match initial_state.grid.height() {
        5 => BurrowState::from_rows(&TARGET_STATE_PART_1),
        7 => BurrowState::from_rows(&TARGET_STATE_PART_2),
        _ => panic!("What part of the puzzle is this?"),
    };
    a_star(
        initial_state,
        &target_state,
        |s| s.successors(),
        |s| s.a_star_heuristic(),
    )
}

pub fn solution_1() -> String {
    let initial_state = BurrowState::from_rows(&INPUT_PART_1);
    least_energy_to_organize(&initial_state).to_string()
}

pub fn solution_2() -> String {
    let initial_state = BurrowState::from_rows(&INPUT_PART_2);
    least_energy_to_organize(&initial_state).to_string()
}

// DEBUG
//impl Display for BurrowState {
//    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//        for r in 0..self.grid.height() {
//            for c in 0..self.grid.width() {
//                write!(f, "{}", self.grid[GridIndex::new(c, r)])?;
//            }
//            writeln!(f)?;
//        }
//        Ok(())
//    }
//}
