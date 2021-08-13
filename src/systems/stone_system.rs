use amethyst::{
    core::transform::Transform,
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    ecs::{Join, Write, Read, ReadStorage, System, SystemData,WriteStorage},

};


use crate::go::{Stone, Go};
use crate::utility::*;

#[derive(SystemDesc)]
pub struct StoneSystem{}

impl StoneSystem {
    pub fn new() -> StoneSystem {
        StoneSystem {
        }
    }
}

impl<'s> System<'s> for StoneSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = Write<'s, Go>;

    fn run(&mut self, (mut game_state): Self::SystemData) {
        let dead_stones = apply_life_and_death_rules_to_board(&mut game_state.board);
        if dead_stones.len() != 0 {
            for i in dead_stones { 
                game_state.board[i].state = 0;
            }
        }
    }
}
