use amethyst::{
    ecs::{System, Read, WriteStorage, ReadStorage, Join, SystemData},
    derive::SystemDesc,
    renderer::SpriteRender,
};

use crate::go::{Stone, Go};

#[derive(SystemDesc)]
pub struct LayoutSystem {}

impl<'s> System<'s> for LayoutSystem {
    type SystemData = (
        Read<'s, Go>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Stone>
    );

    fn run(&mut self, (game_state, mut sprite_renders, stones): Self::SystemData) {
        for (sprite, stone) in (&mut sprite_renders, &stones).join() {
            let stone_index = stone.index;
            let stone_value = game_state.board[stone_index].state;
            sprite.sprite_number = stone_value;
        }
    }
}