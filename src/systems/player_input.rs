use amethyst::{
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
    winit::{VirtualKeyCode}
};

use std::collections::HashSet;

use crate::bangbang::{BangBang, KeyMessage, KeyMessageState};

#[derive(SystemDesc, Default)]
pub struct PlayerInputSystem {
    keys_that_where_down_last_frame: HashSet<VirtualKeyCode>
}

impl<'s> System<'s> for PlayerInputSystem {
    
    type SystemData = (
        Write<'s, BangBang>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut game_state, input): Self::SystemData) {
        let keys_that_are_down : HashSet<VirtualKeyCode> = input.keys_that_are_down().collect();

        for k in keys_that_are_down.difference(&self.keys_that_where_down_last_frame) {
            let key_message = KeyMessage::new(*k, KeyMessageState::Pressed);
            game_state.add_key_message_state(key_message);    
        }

        for k in self.keys_that_where_down_last_frame.difference(&keys_that_are_down) {
            let key_message = KeyMessage::new(*k, KeyMessageState::Released);
            game_state.add_key_message_state(key_message);
        }

        self.keys_that_where_down_last_frame = keys_that_are_down;
    }
}
