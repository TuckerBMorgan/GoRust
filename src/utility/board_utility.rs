
use crate::go::{Stone, Side, StoneData, Go};

pub fn neighbor_indices(index: usize) -> Vec<usize> {
    let index = index as i32;
    let mut up_down : Vec<usize> = vec![index - 9, index + 9].iter().filter(|x| **x >= 0 && **x < 81).filter(|x| ((**x) % 9) == (index%9)).map(|x|(*x) as usize).collect();
    let mut left_right : Vec<usize> = vec![index - 1, index + 1, ].iter().filter(|x| **x >= 0 && **x < 81).filter(|x| ((**x) / 9) == (index/9)).map(|x|(*x) as usize).collect();
    up_down.extend(left_right);
    return up_down;
}

fn grant_life_to_stone(index: usize,  board: &mut [StoneData;81]) {

    if board[index].alive || board[index].state == 0 {
        return;
    }
    board[index].alive = true;

    let index_state = board[index].state;
    let neighbor_indices = neighbor_indices(index);
    for ii in neighbor_indices {
        if board[ii].state == index_state {
            grant_life_to_stone(ii, board);
        }
    }
}

pub fn return_dead_indices(board: &mut [StoneData; 81]) -> Vec<usize> {

    for i in 0..board.len() {
        board[i].alive = false;
    }

    for i in 0..board.len() {
        if board[i].state == 0 {
            let neighbor_indices = neighbor_indices(i);
            for index in neighbor_indices {
                grant_life_to_stone(index, board);
            }
        }
    }

    let mut return_indices = vec![];
    for i in 0..board.len() {
        if board[i].alive == false {
            return_indices.push(i);
        }
    }
    
    return_indices
}