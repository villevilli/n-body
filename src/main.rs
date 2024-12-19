use bevy::prelude::*;
use n_body_platformer::{mouse_camera_control::MouseCameraControl, NBodyPlatformer};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NBodyPlatformer, MouseCameraControl))
        .run();
}
