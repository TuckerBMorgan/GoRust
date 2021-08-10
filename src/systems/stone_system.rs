use amethyst::{
    prelude::*,
    core::transform::Transform,
    input::{InputHandler, ControllerButton, StringBindings, Button},
    core::SystemDesc,
    derive::SystemDesc,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    winit::MouseButton
};


use crate::pong::{Stone, Side};

#[derive(Default, Copy, Clone)]
pub struct StoneData {
    state: usize,
    alive: bool
}

#[derive(SystemDesc)]
pub struct StoneSystem{
    was_mouse_clicked_last_frame: bool,
    turn_number: i32,
    board: [StoneData; 81]
}

impl StoneSystem {
    pub fn new() -> StoneSystem {
        StoneSystem {
            was_mouse_clicked_last_frame: false,
            turn_number: 0,
            board: [StoneData::default(); 81]
        }
    }
}

impl<'s> System<'s> for StoneSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Stone>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut sprite_renders, mut stones, transforms, input): Self::SystemData) {
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            if self.was_mouse_clicked_last_frame == false && input.mouse_button_is_down(MouseButton::Left) {
                self.was_mouse_clicked_last_frame = true;
            }
            else if self.was_mouse_clicked_last_frame && input.mouse_button_is_down(MouseButton::Left) == false {
                self.was_mouse_clicked_last_frame = false;

                for (stone, transform) in (&mut stones, &transforms).join() {

                    let stone_index = stone.index;
                    let translation = transform.translation();
                    //Mouse coordinations are inverse of world cords
                    let mouse_y = 300.0 - mouse_y - 150.0;
                    let mouse_x = mouse_x - 150.0f32;
                    if point_in_rect(mouse_x, mouse_y, translation.x - 16.0, translation.y - 16.0, translation.x + 16.0, translation.y + 16.0) {
                        if self.board[stone_index].state == 0 {
                            if self.turn_number % 2 == 0 {
                                self.board[stone_index].state = 1;
                            }
                            else {
                                self.board[stone_index].state = 2;
                            }                            
                            self.turn_number += 1;
                        }
                    }
                }

                let dead_stones = return_dead_indices(&mut self.board);
                if dead_stones.len() != 0 {
                    for i in dead_stones { 
                        self.board[i].state = 0;
                    }
                    for (stone, sprite_render) in (&stones, &mut sprite_renders).join() {
                        let stone_index = stone.index;
                        sprite_render.sprite_number = self.board[stone_index].state;
                    }
                }
            }
        }
    }
}

fn neighbor_indices(index: usize) -> Vec<usize> {
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

fn return_dead_indices(board: &mut [StoneData; 81]) -> Vec<usize> {

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

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}