use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform, Parent},
    core::math::Vector3,
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::{Camera, Texture, Material, Mesh, MaterialDefaults},
    renderer::shape::Shape,
    renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord},
    ecs::{Component, DenseVecStorage, Entity},
    renderer::palette::{LinSrgba, Srgb},
    renderer::loaders::load_from_linear_rgba,
    winit::{VirtualKeyCode}
};

use crate::systems::{Ball, SphereCollider};

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;


#[derive(Default, Copy, Clone)]
pub struct Player {
    pub stored_data: usize
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub enum KeyMessageState {
    Pressed,
    Released
}

#[derive(Debug)]
pub struct KeyMessage {
    pub keycode: VirtualKeyCode,
    pub state: KeyMessageState
}

impl KeyMessage {
    pub fn new(keycode: VirtualKeyCode, state: KeyMessageState) -> KeyMessage {
        KeyMessage {
            keycode,
            state
        }
    }
}


pub enum CollisionMessageType {
    BulletBall,
    PlayerBall
}

impl Default for CollisionMessageType {
    fn default() -> CollisionMessageType {
        CollisionMessageType::BulletBall
    }
}

pub struct CollisionMessage {
    pub entity_a: Entity,
    pub entity_b: Entity
}

impl CollisionMessage {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionMessage {
        CollisionMessage {
            entity_a,
            entity_b
        }
    }
}

#[derive(Default)]
pub struct KilledEnemyMessage {
}


pub struct BangBang {
    pub key_messages: Vec<KeyMessage>,
    pub collision_messages: Vec<CollisionMessage>,
    pub killed_enemy_messages: Vec<KilledEnemyMessage>
}

impl Default for BangBang {
    fn default() -> BangBang {
        BangBang {
            key_messages: vec![],
            collision_messages: vec![],
            killed_enemy_messages: vec![]
        }
    }
}

impl BangBang {
    pub fn add_key_message_state(&mut self, message: KeyMessage) {
        self.key_messages.push(message);
    }

    pub fn add_collision_message(&mut self, message: CollisionMessage) {
        self.collision_messages.push(message);
    }

    pub fn add_killed_enemy_message(&mut self, message: KilledEnemyMessage) {
        self.killed_enemy_messages.push(message);
    }
}

impl SimpleState for BangBang {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        let mtl = load_material(world);
        let floor = load_floor(world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, -10.0, 0.0);
        transform.set_scale(Vector3::new(100.0, 1.0, 100.0));
        world
            .create_entity()
            .with(transform)
            .with(floor.clone())
            .with(mtl.clone()).build();


        let mtl = load_material(world);
        let mesh = load_mesh(world);


        let light1 : Light = PointLight {
            intensity: 700.0,
            color: Srgb::new(0.78, 0.88, 1.0),
            ..PointLight::default()
        }.into();
        let mut transform = Transform::default();
        transform.set_translation_xyz(6.0, 30.0, -6.0);
    
        world
            .create_entity().
            with(transform).
            with(light1).build();


        let entity_id = world.create_entity().
            with(Transform::default()).
            with(mesh.clone()).
            with(mtl.clone()).
            with(Player::default()).build();

        initialise_camera(world, entity_id);
        world.insert(mtl);
        world.insert(mesh);
    }
}


fn load_floor(world: &mut World) -> Handle<Mesh> {
    let loader = {world.read_resource::<Loader>()};
    let mesh_storage = {world.read_resource::<AssetStorage<Mesh>>()};
    loader.load_from_data(
        Shape::Cube
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
            .into(),
        (),
        &mesh_storage,
    )

}

fn load_mesh(world: &mut World) -> Handle<Mesh> {
    let loader = {world.read_resource::<Loader>()};
    let mesh_storage = {world.read_resource::<AssetStorage<Mesh>>()};
    loader.load_from_data(
        Shape::Sphere(64, 64)
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
            .into(),
        (),
        &mesh_storage,
    )
}

fn load_material(world: &mut World) -> Handle<Material> {
    let loader = world.read_resource::<Loader>();
    let material_storage = world.read_resource::<AssetStorage<Material>>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let albedo = loader.load_from_data(
        load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
        (),
        &texture_storage,
    );

    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    loader.load_from_data(
        Material {
            albedo,
            ..mat_defaults
        },
        (),
        &material_storage,
    )
}

fn initialise_camera(world: &mut World, player: Entity) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 40.0, 0.0);
    transform.face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    transform.prepend_rotation_y_axis(std::f32::consts::PI);
    world
        .create_entity()
        //.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(Camera::standard_3d(ARENA_WIDTH * 1.5, ARENA_HEIGHT * 1.5))
        .with(Parent::new(player))
        .with(transform)
        .build();


}