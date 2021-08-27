
use amethyst:: {
    derive::SystemDesc,
    ecs::{System, Write, SystemData, Entities}
};

use crate::bangbang::{BangBang, KilledEnemyMessage};

#[derive(SystemDesc, Default)]
pub struct CleanupSystem {

}

impl<'s> System<'s> for CleanupSystem {
    type SystemData = (
        Write<'s, BangBang>,
        //todo: loop over all transforms, if they are really far out, delete them
        Entities<'s>
 
    );

    fn run(&mut self, (mut game_state, entities): Self::SystemData) {
        let mut killed_enemy_messages = vec![];
        for collision_message in game_state.collision_messages.iter() {
            let _ = entities.delete(collision_message.entity_a);
            let _ = entities.delete(collision_message.entity_b);
            killed_enemy_messages.push(KilledEnemyMessage::default());

        }
        for kem in killed_enemy_messages {
            game_state.add_killed_enemy_message(kem);
        }

        game_state.collision_messages = vec![];
    }
}