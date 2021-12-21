//! <https://adventofcode.com/2021/day/21>
//!
//! Original implementation for part 1 was a bit different - it required changes, obviously :).
//!
//! Part 2 uses bottom-up memoization. (Sometime I should try top-down, with some automagical
//! library.)

use std::collections::HashMap;

const INPUT_PLAYER_1_STARTING_POSITION: TrackPosition = 3;
const INPUT_PLAYER_2_STARTING_POSITION: TrackPosition = 5;

type TrackPosition = u32;
type PlayerId = usize;

#[derive(Clone, Eq, PartialEq, Hash)]
struct DiracDiceGameState {
    score_per_player: [u32; 2],
    position_per_player: [TrackPosition; 2],
    player_on_turn: PlayerId,
}

fn other_player(player: PlayerId) -> PlayerId {
    (player + 1) % 2
}

impl DiracDiceGameState {
    fn winner(&self, win_threshold: u32) -> Option<PlayerId> {
        if self.score_per_player[0] >= win_threshold {
            Some(0)
        } else if self.score_per_player[1] >= win_threshold {
            Some(1)
        } else {
            None
        }
    }
}

pub fn solution_1() -> String {
    let mut game_state = DiracDiceGameState {
        score_per_player: [0, 0],
        position_per_player: [
            INPUT_PLAYER_1_STARTING_POSITION,
            INPUT_PLAYER_2_STARTING_POSITION,
        ],
        player_on_turn: 0,
    };
    let mut die_state = 1;
    let mut num_rolls = 0;

    while game_state.winner(1000) == None {
        let mut total_rolled = 0;
        for _ in 0..3 {
            total_rolled += die_state;
            die_state = die_state % 100 + 1;
            num_rolls += 1;
        }

        let current_position = game_state.position_per_player[game_state.player_on_turn];
        let new_position = (current_position + total_rolled - 1) % 10 + 1;

        game_state.score_per_player[game_state.player_on_turn] += new_position;
        game_state.position_per_player[game_state.player_on_turn] = new_position;
        game_state.player_on_turn = other_player(game_state.player_on_turn);
    }

    (game_state.score_per_player[other_player(game_state.winner(1000).unwrap())] * num_rolls)
        .to_string()
}

/// Value at index i is the number of universes where player with ID i won.
type DiracDiceOutcome = [u128; 2];

impl DiracDiceGameState {
    fn successors(&self) -> Vec<DiracDiceGameState> {
        let mut successors = Vec::new();
        for roll_1 in 1..=3 {
            for roll_2 in 1..=3 {
                for roll_3 in 1..=3 {
                    let current_position = self.position_per_player[self.player_on_turn];
                    let new_position = (current_position + roll_1 + roll_2 + roll_3 - 1) % 10 + 1;

                    let mut successor = self.clone();
                    successor.score_per_player[successor.player_on_turn] += new_position;
                    successor.position_per_player[successor.player_on_turn] = new_position;
                    successor.player_on_turn = other_player(successor.player_on_turn);
                    successors.push(successor);
                }
            }
        }
        successors
    }
}

/// Returns all pairs of valid, non-winning player scores that add up to the given sum.
fn possible_score_splits(score_sum: i32, win_threshold: i32) -> Vec<[u32; 2]> {
    let mut score_splits = Vec::new();
    for player_1_score in 0..win_threshold {
        let player_2_score = score_sum - player_1_score;
        if (0..win_threshold).contains(&player_2_score) {
            score_splits.push([player_1_score as u32, player_2_score as u32]);
        }
    }
    score_splits
}

fn possible_positions() -> Vec<[TrackPosition; 2]> {
    let mut positions = Vec::new();
    for player_1_position in 1..=10 {
        for player_2_position in 1..=10 {
            positions.push([player_1_position, player_2_position]);
        }
    }
    positions
}

pub fn solution_2() -> String {
    let mut memoization_table: HashMap<DiracDiceGameState, DiracDiceOutcome> = HashMap::new();
    for score_sum in (0..=40).rev() {
        for score_per_player in possible_score_splits(score_sum, 21) {
            for position_per_player in possible_positions() {
                for player_on_turn in [0, 1] {
                    let game_state = DiracDiceGameState {
                        score_per_player,
                        position_per_player,
                        player_on_turn,
                    };

                    let successors = game_state.successors();
                    let successor_outcomes: Vec<DiracDiceOutcome> = successors
                        .iter()
                        .map(|state| match state.winner(21) {
                            Some(player) => {
                                // Game is over, so there is only 1 outcome, winner won.
                                let mut outcome = [0, 0];
                                outcome[player] = 1;
                                outcome
                            }
                            None => memoization_table[state],
                        })
                        .collect();

                    let total_player_1_wins = successor_outcomes.iter().map(|o| o[0]).sum();
                    let total_player_2_wins = successor_outcomes.iter().map(|o| o[1]).sum();
                    let outcome = [total_player_1_wins, total_player_2_wins];

                    memoization_table.insert(game_state, outcome);
                }
            }
        }
    }

    let initial_state = DiracDiceGameState {
        score_per_player: [0, 0],
        position_per_player: [
            INPUT_PLAYER_1_STARTING_POSITION,
            INPUT_PLAYER_2_STARTING_POSITION,
        ],
        player_on_turn: 0,
    };
    let game_outcome = memoization_table[&initial_state];

    game_outcome.iter().max().unwrap().to_string()
}
