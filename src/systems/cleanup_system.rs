
use amethyst:: {
    derive::SystemDesc,
    ecs::{System, Write, SystemData, Entities}
};

use crate::bangbang::{BangBang};

#[derive(SystemDesc, Default)]
pub struct CleanupSystem {

}

impl<'s> System<'s> for CleanupSystem {
    type SystemData = (
        Write<'s, BangBang>,
        //todo: loop over all transforms, if they are really far out, delete them
        Entities<'s>
 
    );

    fn run(&mut self, (game_state, entities): Self::SystemData) {
        for collision_message in game_state.collision_messages.iter() {
            let _ = entities.delete(collision_message.entity_a);
            let _ = entities.delete(collision_message.entity_b);
        }
    }
}