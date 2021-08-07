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

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 32.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
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

        //self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        let sr = load_stone_sprite_sheet(world);
        initialise_stones(world, sr);
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


fn initialise_stones(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    for i in 0.. 3 {
        let mut left_transform = Transform::default();
        left_transform.set_translation_xyz(PADDLE_WIDTH * i as f32, ARENA_WIDTH / 2.0f32, 0.0);
        let scale = Vector3::new(0.5, 0.5, 1.0);
        left_transform.set_scale(scale);

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Stone::new(Side::Left))
            .with(left_transform)
            .build();
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
            "texture/go_sprite_sheet.png",
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