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
pub struct Ball {
    pub index: usize
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}


#[derive(SystemDesc, Default)]
pub struct BallSystem {
    
}


impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadExpect<'s, Handle<Mesh>>,
        ReadExpect<'s, Handle<Material>>,
        
    );

    fn run(&mut self, (mut balls, mut transforms, player, mesh, material): Self::SystemData) {
        
        let mut player_position = Vector3::new(0.0, 0.0, 0.0);
        for (_player, transform) in (&player, &mut transforms).join() {
            player_position = transform.translation().clone();
        }
        let ball_speed = 0.01f32;
        for (ball, transform) in (&mut balls, &mut transforms).join() {

            let direction = (player_position - transform.translation()).normalize() * 0.01f32;
            transform.set_translation_xyz(transform.translation().x + direction.x, 
                                          transform.translation().y + direction.y, 
                                          transform.translation().z + direction.z);
        }
    }

}