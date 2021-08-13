use amethyst::{
    ecs::{System, Write},
};

use rand::seq::SliceRandom;

use crate::utility::*;
use crate::go::{StoneData, Go};
#[derive(Default)]
pub struct AISystem;

impl<'s> System<'s> for AISystem {

    type SystemData = Write<'s, Go>;

    fn run(&mut self, mut game_state: Self::SystemData) {
        if game_state.turn_number % 2 == 1 {
            let all_possible_moves = find_all_possible_moves(&mut game_state.board);
            let mut next_possible_states = vec![];
            for index in &all_possible_moves {
                game_state.board[*index].state = 2;
                next_possible_states.push(generate_next_board_state(&game_state.board));
                game_state.board[*index].state = 0;
            }
            //Take all of the board, 
            let mut index_of_move_to_make = next_possible_states.iter()
                //convert them into a single value
                .map(|x| value_board(&x, 2))
                //Give them each their index
                .enumerate()
                //Convert that into a tuple for sorting
                .map(|(i, x)|(i, x)).collect::<Vec<(usize, isize)>>();

            //Sort all of the tuples by their value
            index_of_move_to_make.sort_by(|a, b| a.1.cmp(&b.1));
            let move_to_make = select_move_to_make(&mut index_of_move_to_make);
            game_state.board[all_possible_moves[move_to_make]].state = 2;
            game_state.turn_number += 1;
        }
    }
}

fn select_move_to_make(board_values: &mut Vec<(usize, isize)>) -> usize {
    let mut top_values = vec![];
    top_values.push(board_values.remove(0));
    for i in 1..board_values.len() {
        if board_values[i].1 == top_values[0].1 {
            top_values.push(board_values[i]);
        }
        else {
            break;
        }

    }

    let possible_indexes : Vec<usize> = top_values.iter().map(|(i, _x)| *i).collect();
    return *possible_indexes.choose(&mut rand::thread_rng()).unwrap();
}

fn value_board(board: &[StoneData; 81], side: usize) -> isize {
    let mut board_value = 0;

    for i in 0..board.len() {
        if board[i].state == side {
            board_value += 1;
        }
        else if board[i].state != 0 {
            board_value -= 1;
        }
    }
    return board_value;
}


fn find_all_possible_moves(game_board: &mut [StoneData; 81]) -> Vec<usize> {
    let mut possible_indexes = vec![];
    for i in 0..game_board.len() {
        if game_board[i].state == 0 {
            possible_indexes.push(i);
        }
    }
    return possible_indexes;
}