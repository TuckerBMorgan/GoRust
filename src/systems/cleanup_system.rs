
use amethyst:: {
    derive::SystemDesc,
    ecs::{System, SystemData,Write,  Entities, WriteExpect}
};

use crate::bangbang::{BangBang, KilledEnemyMessage, CollisionMessageType, GameStateForSystem};

#[derive(SystemDesc, Default)]
pub struct CleanupSystem {
}

impl<'s> System<'s> for CleanupSystem {
    type SystemData = (
        WriteExpect<'s, BangBang>,
        //todo: loop over all transforms, if they are really far out, delete them
        Entities<'s>,
        Write<'s, GameStateForSystem>
    );

    fn run(&mut self, (mut game_state, entities, mut game_state_for_system): Self::SystemData) {
        let mut killed_enemy_messages = vec![];
        for collision_message in game_state.collision_messages.iter() {
            match collision_message.collision_message_type {
                CollisionMessageType::BulletEnemy => {
                    let _ = entities.delete(collision_message.entity_a);
                    let _ = entities.delete(collision_message.entity_b);
                    killed_enemy_messages.push(KilledEnemyMessage::default());        
                },
                CollisionMessageType::PlayerEnemy => {
                    //now we need to "reset" the gamedaddddddddd, so, dump shit yo
                    game_state_for_system.should_reset = true;
                    break;
                }
            }

        }
        for kem in killed_enemy_messages {
            game_state.add_killed_enemy_message(kem);
        }

        game_state.collision_messages = vec![];
    }
}