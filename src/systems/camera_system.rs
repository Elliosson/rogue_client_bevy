// ici on va lire les donne du network sur notre position et centrer le tou en consequant

use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (WriteStorage<'s, Transform>, WriteStorage<'s, Camera>);

    fn run(&mut self, (mut transforms, cameras): Self::SystemData) {
        /* for (transform, camera) in (&mut transforms, &cameras).join() {
            //TODO set camera with my position
            transform.set_translation_xyz(500., 500., 1.);
        }*/
    }
}
