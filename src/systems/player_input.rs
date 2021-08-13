use amethyst::{
    core::transform::Transform,
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    ecs::{Join, Write, Read, ReadStorage, System, SystemData,WriteStorage},
    winit::MouseButton
};

use crate::go::{Stone, Go};

#[derive(SystemDesc, Default)]
pub struct PlayerInputSystem {
    was_mouse_clicked_last_frame: bool
}

impl<'s> System<'s> for PlayerInputSystem {
    
    type SystemData = (
        Write<'s, Go>,
        WriteStorage<'s, Stone>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut game_state, mut stones, transforms, input): Self::SystemData) {
        if game_state.turn_number % 2 != 0 {
            //It is the other persons turn to play a match
            return;
        }
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            if self.was_mouse_clicked_last_frame == false && input.mouse_button_is_down(MouseButton::Left) {
                self.was_mouse_clicked_last_frame = true;
            }
            else if self.was_mouse_clicked_last_frame && input.mouse_button_is_down(MouseButton::Left) == false {
                self.was_mouse_clicked_last_frame = false;

                for (stone, transform) in (&mut stones, &transforms).join() {

                    let stone_index = stone.index;
                    let translation = transform.translation();
                    //Mouse coordinations are inverse of world cords
                    let mouse_y = 300.0 - mouse_y - 150.0;
                    let mouse_x = mouse_x - 150.0f32;
                    if point_in_circle(mouse_x, mouse_y, translation.x, translation.y, 15.0) {
                        if game_state.board[stone_index].state == 0 {
                            game_state.board[stone_index].state = 1;
                            game_state.turn_number += 1;
                        }
                    }
                }
            }
        }
    }
}

fn point_in_circle(x: f32, y: f32, center_x: f32, center_y: f32, radius: f32) -> bool {
    let radius_squared = radius * radius;
    let x_dif = (x - center_x).powf(2.0);
    let y_dif = (y - center_y).powf(2.0);
    return radius_squared > (x_dif + y_dif);
}
