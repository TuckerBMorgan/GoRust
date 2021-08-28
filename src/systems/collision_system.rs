use amethyst:: {
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage},
    ecs::{System, ReadStorage, Write, Join, SystemData, Entities}
};


use crate::bangbang::{BangBang, CollisionMessage, CollisionMessageType, Player};
use crate::systems::{Bullet, Enemy};

#[derive(Default)]
pub struct SphereCollider {
    radius: f32
}

impl SphereCollider {

    pub fn new(radius: f32) -> SphereCollider {
        SphereCollider {
            radius
        }
    }
}

impl Component for SphereCollider {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc, Default)]
pub struct CollisionSystem {

}

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Write<'s, BangBang>,
        ReadStorage<'s, SphereCollider>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Bullet>
    );

    fn run(&mut self, (mut game_state, sphere_colliders, transforms, player, entities, enemies, bullets): Self::SystemData) {

        for (sphere_collider, transform, entity, _) in (&sphere_colliders, &transforms, &entities, &bullets).join() {
            for (other_sphere_collider, other_transform, other_entity, _) in (&sphere_colliders, &transforms, &entities, &enemies).join() {
                if entity.id() == other_entity.id() {
                    continue;
                }
                let distance = (transform.translation() - other_transform.translation()).magnitude();
                if distance < sphere_collider.radius + other_sphere_collider.radius {
                    let collision_message = CollisionMessage::new(entity.clone(), other_entity.clone(), CollisionMessageType::BulletEnemy);
                    game_state.add_collision_message(collision_message, );
                }
            }
        }
        
        for (sphere_collider, transform, entity, _) in (&sphere_colliders, &transforms, &entities, &enemies).join() {
            for (player_sphere_collider, player_transform, player_entity, _) in (&sphere_colliders, &transforms, &entities, &player).join() {
                let distance = (transform.translation() - player_transform.translation()).magnitude();
                if distance < sphere_collider.radius + player_sphere_collider.radius {
                    let collision_message = CollisionMessage::new(entity.clone(), player_entity.clone(), CollisionMessageType::PlayerEnemy);
                    game_state.add_collision_message(collision_message);
                }
            }
        }
        
    }
}
