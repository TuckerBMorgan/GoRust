use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::math::{Vector3
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Component, DenseVecStorage, Entity},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

use amethyst::core::timing::Time;

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 32.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Empty,
    White,
    Black

}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Pong{
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>
}


pub struct Stone {
    pub side: Side,
}

impl Component for Stone {
    type Storage = DenseVecStorage<Self>;
}

impl Stone {
    fn new(side: Side) -> Stone {
        Stone {
            side
        }
    }
}

impl SimpleState for Pong {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Stone>();
        
        let sr = load_stone_sprite_sheet(world);
        initialise_stones(world, sr.clone());
        initialise_camera(world);
        self.sprite_sheet_handle.replace(sr);
        
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.0, ARENA_HEIGHT * 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


fn initialise_stones(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 2);
    for i in 0.. 9 {
        for y in 0..9 {

            let mut left_transform = Transform::default();
            left_transform.set_translation_xyz((16.0 +  i as f32 * 32.0) - ((9.0 * 32.0) / 2.0), (16.0 +  y as f32 * 32.0) - ((9.0 * 32.0) / 2.0), 0.0);

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Stone::new(Side::Empty))
                .with(left_transform)
                .build();
        }
    }    
}

fn load_stone_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/go_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/go_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )

    //...
}