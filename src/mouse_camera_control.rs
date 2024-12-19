use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseMotion},
    prelude::*,
};

const SENSITIVITY: f32 = 3.0;

pub struct MouseCameraControl;

#[derive(Component)]
struct MainCameraMarker;

impl Plugin for MouseCameraControl {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_mouse_input);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCameraMarker));
}

fn handle_mouse_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut camera_query: Query<(&mut Camera2d, &mut Transform), With<MainCameraMarker>>,
) {
    //It's okay to panic if there are two main cameras
    let (camera, mut transform) = camera_query.single_mut();

    if mouse_buttons.pressed(MouseButton::Right) {
        if let Some(motion) = evr_motion.read().next() {
            transform.translation.x -= motion.delta.x * SENSITIVITY;
            transform.translation.y += motion.delta.y * SENSITIVITY;
        }
    }
}
