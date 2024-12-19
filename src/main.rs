use bevy::prelude::*;
use n_body_platformer::NBodyPlatformer;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NBodyPlatformer))
        .run();
}
