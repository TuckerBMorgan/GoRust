use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::{Srgb},
    ecs::{System, WriteStorage, ReadStorage, Join, SystemData, DenseVecStorage, Component, LazyUpdate, Write, ReadExpect},
    renderer::{Mesh, Material},
    derive::SystemDesc,
    assets::Handle
};

use rand::Rng;
use std::collections::HashMap;
use crate::bangbang::{BangBang, Player};
use crate::systems::{Ball, Rusher};
use crate::systems::SphereCollider;

#[derive(Default, Copy, Clone)]
pub struct Enemy {
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}


#[derive(SystemDesc)]
pub struct SpawnerSystem {
    current_desired_amount_of_enemies: usize,
    number_of_spawned_enemies: usize
}

impl Default for SpawnerSystem {
    fn default() -> SpawnerSystem {
        SpawnerSystem {
            current_desired_amount_of_enemies: 10,
            number_of_spawned_enemies: 0
        }
    }
}

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Write<'s, BangBang>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Write<'s, LazyUpdate>,
        ReadExpect<'s,HashMap<String, Handle<Mesh>>>,
        ReadExpect<'s, Handle<Material>>,
    );

    fn run(&mut self, (mut game_state, player, mut transforms, lz, mesh, material): Self::SystemData) {

        let mut player_position = Vector3::new(0.0, 0.0, 0.0);
        for (_player, transform) in (&player, &mut transforms).join() {
            player_position = transform.translation().clone();
        }

        let enemies_killed_last_frame = game_state.killed_enemy_messages.len();
        self.number_of_spawned_enemies -= enemies_killed_last_frame;
        let mut rng = rand::thread_rng();
        let balls_to_create = self.current_desired_amount_of_enemies - self.number_of_spawned_enemies;
        for i in 0..balls_to_create {
            let mut transform = Transform::default();

            let offset = ( 10 * rng.gen_range(0, 36)) as f32;
            let x_offset = (offset * (3.14 / 180.0)).sin();
            let y_offset = (offset * (3.14 / 180.0)).cos();
            transform.set_translation_xyz(player_position.x + x_offset * 20.0, 0.0, player_position.z + y_offset * 20.0);
            
            let light1 : Light = PointLight {
                intensity: 10.0,
                color: Srgb::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
                ..PointLight::default()
            }.into();

            let at_c = mesh["sphere"].clone();
            let mat_c = material.clone();
            if rng.gen_range(0.0, 1.0) > 0.1 {
                lz.exec_mut(move|world: &mut World|{
                    world.create_entity().
                        with(transform).
                        with(at_c.clone()).
                        with(mat_c.clone()).
                        with(light1).
                        with(Enemy::default()).
                        with(SphereCollider::new(1.0)).
                        with(Ball{index: i}).build();

                });
            }
            else {
                lz.exec_mut(move|world: &mut World|{
                    world.create_entity().
                        with(transform).
                        with(at_c.clone()).
                        with(mat_c.clone()).
                        with(light1).
                        with(Enemy::default()).
                        with(SphereCollider::new(1.0)).
                        with(Rusher{index: i}).build();

                });
            }
            self.number_of_spawned_enemies += 1;
        }
        game_state.killed_enemy_messages = vec![];
    }

}