use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{System,  WriteStorage, Join, SystemData, DenseVecStorage, Component},
    derive::SystemDesc,
};

const BULLET_SPEED : f32 = 0.5;

#[derive(Copy, Clone)]
pub struct Bullet {
    direction: Vector3<f32>
}

impl Default for Bullet {
    fn default() -> Bullet {
        Bullet {
            direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Bullet {
    pub fn new(direction: Vector3<f32>) -> Bullet {
        Bullet {
            direction
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc, Default)]
pub struct BulletSystem {
    pub ticks_seen: usize
}

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (mut bullets, mut transforms): Self::SystemData) {
        for (bullet, transform) in (&mut bullets, &mut transforms).join() {
            let mut update_vector = bullet.direction * -BULLET_SPEED;
            transform.set_translation_xyz(transform.translation().x + update_vector.x,
                                         0.0,
                                         transform.translation().z + update_vector.z);
        }
    }
}
