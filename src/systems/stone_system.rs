use amethyst::{
    prelude::*,
    core::transform::Transform,
    input::{InputHandler, ControllerButton, StringBindings, Button},
    core::SystemDesc,
    derive::SystemDesc,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    winit::MouseButton
};


use crate::pong::{Stone, Side};

#[derive(SystemDesc, Default)]
pub struct StoneSystem{
    was_mouse_clicked_last_frame: bool
}

impl<'s> System<'s> for StoneSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Stone>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut sprite_renders, mut stones, transforms, input): Self::SystemData) {



        if let Some((mouse_x, mouse_y)) = input.mouse_position() {

            if self.was_mouse_clicked_last_frame == false && input.mouse_button_is_down(MouseButton::Left) {
                self.was_mouse_clicked_last_frame = true;
            }
            else if self.was_mouse_clicked_last_frame && input.mouse_button_is_down(MouseButton::Left) == false {
                self.was_mouse_clicked_last_frame = false;

                for (stone, sprite_render, transform) in (&mut stones, &mut sprite_renders, &transforms).join() {
                    let translation = transform.translation();
                    //Mouse coordinations are inverse of world cords
                    let mouse_y = 300.0 - mouse_y - 150.0;
                    let mouse_x = mouse_x - 150.0f32;
                    if point_in_rect(mouse_x, mouse_y, translation.x - 16.0, translation.y - 16.0, translation.x + 16.0, translation.y + 16.0) {
                        stone.side = Side::White;
                    }
                    match stone.side {
                        Side::White => {
                            sprite_render.sprite_number = 1;
                        },
                        Side::Black => {
                            sprite_render.sprite_number = 0;
                        },
                        Side::Empty => {
                            sprite_render.sprite_number = 2;
                        }
                    }
                }



            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}