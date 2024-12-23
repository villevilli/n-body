use bevy::prelude::*;
use n_body_platformer::commands::DevCommandline;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevCommandline)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
