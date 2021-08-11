use amethyst::{
    ecs::{System, Write},
};

use crate::go::{StoneData, Go};
pub struct AISystem {

}

impl<'s> System<'s> for AISystem {

    type SystemData = Write<'s, Go>;

    fn run(&mut self, mut game_state: Self::SystemData) {
        if game_state.turn_number % 2 == 1 {
            let all_possible_moves = find_all_possible_moves(&mut game_state.board);
            
        }
    }
}

fn value_board(board: &mut [StoneData; 81], side: usize) {
    for i in 0..board.len() {

    }
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