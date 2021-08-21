use amethyst::{
    core::transform::Transform,
    ecs::{System, WriteStorage, Join, SystemData, DenseVecStorage, Component},
    derive::SystemDesc,
};

#[derive(Default, Copy, Clone)]
pub struct Ball {
    pub index: usize
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}


#[derive(SystemDesc, Default)]
pub struct BallSystem {
    pub ticks_seen: usize
}

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let offset = 90.0 * ball.index as f32;
            let offset = offset + self.ticks_seen as f32 / 2.0f32;
            let x_offset = (offset * (3.14 / 180.0)).sin();
            let y_offset = (offset * (3.14 / 180.0)).cos();
            transform.set_translation_xyz(x_offset * 20.0, 0.0, y_offset * 20.0);
        }
        self.ticks_seen += 1;
    }

}