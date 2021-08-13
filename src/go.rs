use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Component, DenseVecStorage},
};


pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

#[derive(Default, Copy, Clone)]
pub struct StoneData {
    pub state: usize,
    pub alive: bool
}


pub struct Go {
    pub ball_spawn_timer: Option<f32>,
    pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub board: [StoneData; 81],
    pub turn_number: i32
}

impl Default for Go {
    fn default() -> Go {
        Go {
            ball_spawn_timer: None,
            sprite_sheet_handle: None,
            board: [StoneData::default(); 81],
            turn_number: 0
        }
    }
}

impl SimpleState for Go {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Stone>();
        
        let sr = load_stone_sprite_sheet(world);
        initialise_stones(world, sr.clone());
        initialise_camera(world);
        self.sprite_sheet_handle.replace(sr);
        
    }

}

pub struct Stone {
    pub index: usize
}

impl Component for Stone {
    type Storage = DenseVecStorage<Self>;
}

impl Stone {
    fn new(index: usize) -> Stone {
        Stone {
            index
        }
    }
}
/*
pub struct Message {
    pub value: usize
}

impl Message {
    pub fn new() -> Message {
        Message {
            value: 0
        }
    }
}

impl Component for Message {
    type Storage = DenseVecStorage<Self>;
}
*/

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
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    for i in 0.. 9 {
        for y in 0..9 {

            let mut left_transform = Transform::default();
            left_transform.set_translation_xyz((16.0 +  i as f32 * 32.0) - ((9.0 * 32.0) / 2.0), (16.0 +  y as f32 * 32.0) - ((9.0 * 32.0) / 2.0), 0.0);

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Stone::new(i * 9 + y))
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