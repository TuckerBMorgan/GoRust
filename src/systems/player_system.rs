use amethyst::{
    core::timing::Time,
    core::math::Vector3,
    core::transform::Transform,
    ecs::{System, WriteStorage, Write, Read, Join, SystemData, LazyUpdate, ReadExpect},
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    prelude::*,
    winit::VirtualKeyCode,
    renderer::{Mesh, Material},
    assets::Handle
};

use std::collections::HashMap;

use crate::bangbang::{Player, BangBang, KeyMessageState};
use crate::systems::{SphereCollider, Bullet};

const PLAYER_SPEED : f32 = 0.1f32;
const RELOAD_SPEED : f32 = 0.2532;

#[derive(SystemDesc, Default)]
pub struct PlayerSystem {
    pub ticks_seen: usize,
    pub player_is_moving: bool,
    pub left_right_axis: f32,
    pub up_down_axis: f32,
    next_bullet_timer: f32
}


impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Write<'s, BangBang>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, HashMap<String, Handle<Mesh>>>,
        ReadExpect<'s, Handle<Material>>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut game_state, mut players, mut transforms, lz, at, mat, time, input): Self::SystemData) {
        let mut create_bullet = false;
        let mut bullet_direction = Vector3::new(0.0, 0.0, 0.0);
        if input.axis_value("shoot_up_down").unwrap() != 0.0  || input.axis_value("shoot_left_right").unwrap() != 0.0f32 {
            bullet_direction.x = input.axis_value("shoot_left_right").unwrap();
            bullet_direction.z = input.axis_value("shoot_up_down").unwrap();
            create_bullet = true;
        }

        for (_player, transform) in (&mut players, &mut transforms).join() {
            transform.set_translation_xyz(transform.translation().x + input.axis_value("left_right").unwrap() * PLAYER_SPEED, 
                                          0.0,
                                          transform.translation().z + input.axis_value("up_down").unwrap()  * PLAYER_SPEED);
            if create_bullet && self.next_bullet_timer == 0.0 {
                let mut bullet_transform = Transform::default();
                bullet_transform.set_translation_xyz(transform.translation().x, transform.translation().y, transform.translation().z);
                let at_c = at["sphere"].clone();
                let mat_c = mat.clone();
                lz.exec_mut(move|world: &mut World| {
                    world.create_entity()
                    .with(bullet_transform)
                    .with(at_c)
                    .with(mat_c)
                    .with(Bullet::new(bullet_direction))
                    .with(SphereCollider::new(1.0)).build();
                });
                self.next_bullet_timer = RELOAD_SPEED;
            }
        }

        if self.next_bullet_timer > 0.0f32 {
            self.next_bullet_timer -= 2.0 * time.delta_seconds();
            if self.next_bullet_timer <= 0.0f32 {
                self.next_bullet_timer = 0.0f32;
            }
        }
        
        game_state.key_messages = vec![];
        self.ticks_seen += 1;
    }

}