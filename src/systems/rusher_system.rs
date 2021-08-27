use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::{LinSrgba, Srgb},
    ecs::{System, WriteStorage, ReadStorage, Read, Join, SystemData, DenseVecStorage, Component, LazyUpdate, Write, ReadExpect},
    renderer::{Mesh, Material},
    derive::SystemDesc,
    assets::Handle
};
use rand::Rng;
const TOTAL_NUMBER_ENEMIES : usize = 5;

use crate::bangbang::Player;
use crate::systems::SphereCollider;

#[derive(Default, Copy, Clone)]
pub struct Rusher {
    pub index: usize
}

impl Component for Rusher {
    type Storage = DenseVecStorage<Self>;
}


#[derive(SystemDesc, Default)]
pub struct RusherSystem {
}


impl<'s> System<'s> for RusherSystem {
    type SystemData = (
        WriteStorage<'s, Rusher>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadExpect<'s, Handle<Mesh>>,
        ReadExpect<'s, Handle<Material>>,  
    );

    fn run(&mut self, (mut rushers, mut transforms, player, mesh, material): Self::SystemData) {
        
        let mut player_position = Vector3::new(0.0, 0.0, 0.0);
        for (_player, transform) in (&player, &mut transforms).join() {
            player_position = transform.translation().clone();
        }
        let ball_speed = 0.1f32;
        for (rusher, transform) in (&mut rushers, &mut transforms).join() {

            let direction = (player_position - transform.translation()).normalize() * ball_speed;
            transform.set_translation_xyz(transform.translation().x + direction.x, 
                                          transform.translation().y + direction.y, 
                                          transform.translation().z + direction.z);
        }
    }

}