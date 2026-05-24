use specs::System;

pub struct SpriteRenderer;

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        todo!()
    }
}
