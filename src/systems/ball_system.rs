use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::{System, WriteStorage, ReadStorage, Join, SystemData, DenseVecStorage, Component},
    derive::SystemDesc
};

const BALL_BASE_SPEED : f32 = 0.05f32;

use crate::bangbang::Player;

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
        ReadStorage<'s, Player>        
    );

    fn run(&mut self, (mut balls, mut transforms, player) : Self::SystemData) {
        
        let mut player_position = Vector3::new(0.0, 0.0, 0.0);
        for (_player, transform) in (&player, &mut transforms).join() {
            player_position = transform.translation().clone();
        }
        for (_ball, transform) in (&mut balls, &mut transforms).join() {

            let direction = (player_position - transform.translation()).normalize() * BALL_BASE_SPEED;
            transform.set_translation_xyz(transform.translation().x + direction.x, 
                                          transform.translation().y + direction.y, 
                                          transform.translation().z + direction.z);
        }
    }

}