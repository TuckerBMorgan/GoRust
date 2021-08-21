use amethyst::{
    core::transform::Transform,
    ecs::{System, WriteStorage, Write, Join, SystemData, LazyUpdate, ReadExpect},
    derive::SystemDesc,
    prelude::*,
    winit::VirtualKeyCode,
    renderer::{Mesh, Material},
    assets::Handle
};

use crate::bangbang::{Player, BangBang, KeyMessageState};
use crate::systems::{SphereCollider, Bullet};

const PLAYER_SPEED : f32 = 0.1f32;

#[derive(SystemDesc, Default)]
pub struct PlayerSystem {
    pub ticks_seen: usize,
    pub player_is_moving: bool,
    pub left_right_axis: f32,
    pub up_down_axis: f32
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Write<'s, BangBang>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Write<'s, LazyUpdate>,
        ReadExpect<'s, Handle<Mesh>>,
        ReadExpect<'s, Handle<Material>>
    );

    fn run(&mut self, (mut game_state, mut players, mut transforms, lz, at, mat): Self::SystemData) {
        let mut create_bullet = false;
        let mut bullet_direction = 0;
        if game_state.key_messages.len() != 0 {
            for message in &game_state.key_messages {
                match message.state {
                    KeyMessageState::Pressed => {
                        if message.keycode == VirtualKeyCode::W {
                            self.up_down_axis = -1.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::S {
                            self.up_down_axis = 1.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::A {
                            self.left_right_axis = -1.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::D {
                            self.left_right_axis = 1.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::Up {
                            create_bullet = true;
                            bullet_direction = 0;
                        }
                        else if message.keycode == VirtualKeyCode::Right {
                            create_bullet = true;
                            bullet_direction = 1;
                        }
                        else if message.keycode == VirtualKeyCode::Down {
                            create_bullet = true;
                            bullet_direction = 2;
                        }
                        else if message.keycode == VirtualKeyCode::Left {
                            create_bullet = true;
                            bullet_direction = 3;
                        }
                    },
                    KeyMessageState::Released => {
                        if message.keycode == VirtualKeyCode::W {
                            self.up_down_axis = 0.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::S {
                            self.up_down_axis = 0.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::A {
                            self.left_right_axis = 0.0f32;
                        }
                        else if message.keycode == VirtualKeyCode::D {
                            self.left_right_axis = 0.0f32;
                        }
                    }
                }
            }
        }

        for (_player, transform) in (&mut players, &mut transforms).join() {
            transform.set_translation_xyz(transform.translation().x + self.left_right_axis * PLAYER_SPEED, 
                                          0.0,
                                          transform.translation().z + self.up_down_axis * PLAYER_SPEED);
            if create_bullet {
                let mut bullet_transform = Transform::default();
                bullet_transform.set_translation_xyz(transform.translation().x, transform.translation().y, transform.translation().z);
                let at_c = at.clone();
                let mat_c = mat.clone();
                lz.exec_mut(move|world: &mut World| {
                    world.create_entity()
                    .with(bullet_transform)
                    .with(at_c)
                    .with(mat_c)
                    .with(Bullet::new(bullet_direction))
                    .with(SphereCollider::new(1.0)).build();
                });
            }
        }
        
        game_state.key_messages = vec![];
        self.ticks_seen += 1;
    }

}