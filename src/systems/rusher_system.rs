use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::{System, WriteStorage, ReadStorage, Join, SystemData, DenseVecStorage, Component},
    derive::SystemDesc,
};

const RUSHER_BASE_SPEED : f32 = 0.18f32;
use crate::bangbang::Player;

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
        ReadStorage<'s, Player>
    );

    fn run(&mut self, (mut rushers, mut transforms, player): Self::SystemData) {
        
        let mut player_position = Vector3::new(0.0, 0.0, 0.0);
        for (_player, transform) in (&player, &mut transforms).join() {
            player_position = transform.translation().clone();
        }

        for (_rusher, transform) in (&mut rushers, &mut transforms).join() {

            let direction = (player_position - transform.translation()).normalize() * RUSHER_BASE_SPEED;
            transform.set_translation_xyz(transform.translation().x + direction.x, 
                                          transform.translation().y + direction.y, 
                                          transform.translation().z + direction.z);
        }
    }

}