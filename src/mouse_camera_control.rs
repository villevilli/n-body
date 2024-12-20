use std::ops::Not;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::PrimaryWindow,
};

const SENSITIVITY: f32 = 0.10;

#[derive(Component)]
struct MainCameraMarker;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AlwaysRunning;

#[derive(Default)]
pub struct MouseCameraControl<S: States> {
    pub running_state: S,
}

impl MouseCameraControl<AlwaysRunning> {
    pub const fn always_on() -> MouseCameraControl<AlwaysRunning> {
        MouseCameraControl {
            running_state: AlwaysRunning,
        }
    }
}

impl<S> Plugin for MouseCameraControl<S>
where
    S: States,
{
    fn build(&self, app: &mut App) {
        app.insert_state(AlwaysRunning);
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            camera_mouse_control.run_if(in_state(self.running_state.clone())),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCameraMarker));
}

fn camera_mouse_control(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCameraMarker>>,
) {
    //It's okay to panic if there are two main cameras
    let (mut projection, mut transform) = camera_query.single_mut();

    //It's also okay to panic with two primary windows
    let window = window_query.single();

    if mouse_buttons.pressed(MouseButton::Right) {
        let mut translation = mouse_motion.delta.extend(0.0);

        //Since we want to "Drag" the screen we must negate the x
        translation.x *= -1.0;
        transform.translation += translation * projection.scale;
    }

    if mouse_scroll.delta.y != 0.0 {
        let window_middle: Vec2 = window.physical_size().as_vec2() * 0.5;
        let zoom_amount = projection.scale * mouse_scroll.delta.y * SENSITIVITY;

        //We translate the camera so it zooms with the mouse in the middle
        let mut camera_translation = window
            .cursor_position()
            .map_or(window_middle, |v| v - window_middle);

        //We have to negate the y so that the zoom is towards the right position
        camera_translation.y *= -1.0;

        println!("Mouse Positino: {}", camera_translation);

        transform.translation += camera_translation.extend(0.0) * zoom_amount;
        projection.scale -= zoom_amount;
    }
}
