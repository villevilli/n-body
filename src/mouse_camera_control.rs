use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::PrimaryWindow,
};

const SENSITIVITY: f32 = 0.10;

#[derive(Component, Default)]
pub struct MainCameraMarker;

#[derive(Default)]
pub struct MouseCameraControl<S: States> {
    pub running_state: S,
    pub camera_settings: CameraSettings,
}

#[derive(Clone)]
pub struct CameraSettings {
    pub pos: Vec2,
    pub zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            zoom: 1.0,
        }
    }
}

#[derive(Event)]
pub struct CameraSettingsChange(CameraSettings);

impl<S> Plugin for MouseCameraControl<S>
where
    S: States,
{
    fn build(&self, app: &mut App) {
        app.add_event::<CameraSettingsChange>();
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                camera_mouse_control.run_if(in_state(self.running_state.clone())),
                set_camera_position,
            ),
        );
        let mut a = IntoSystem::into_system(initial_set_camera_position);
        a.initialize(app.world_mut());
        a.run(self.camera_settings.clone(), app.world_mut());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCameraMarker));
}

fn initial_set_camera_position(
    camera_settings: In<CameraSettings>,
    mut ew_set_camera_pos: EventWriter<CameraSettingsChange>,
) {
    ew_set_camera_pos.write(CameraSettingsChange(camera_settings.0));
}

fn set_camera_position(
    mut camera_query: Query<(&mut Projection, &mut Transform), With<MainCameraMarker>>,
    mut ev_set_camera_pos: EventReader<CameraSettingsChange>,
) {
    for set_camera_pos in ev_set_camera_pos.read() {
        let (mut camera_projection, mut transform) =
            camera_query.single_mut().expect("Missing main camera");

        let Projection::Orthographic(ref mut camera_projection) = *camera_projection else {
            panic!("Non ortographic projection not supported")
        };

        transform.translation = set_camera_pos.0.pos.extend(transform.translation.z);
        camera_projection.scale = set_camera_pos.0.zoom
    }
}

fn camera_mouse_control(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Projection, &mut Transform), With<MainCameraMarker>>,
) {
    //It's okay to panic if there are two main cameras
    let (mut projection, mut transform) =
        camera_query.single_mut().expect("Multiple primary cameras");

    let Projection::Orthographic(ref mut projection) = *projection else {
        panic!("Non ortographic projection not supported")
    };

    //It's also okay to panic with two primary windows
    let window = window_query.single().expect("Multiple primary windows");

    if mouse_buttons.pressed(MouseButton::Right) {
        let mut translation = mouse_motion.delta.extend(0.0);

        //Since we want to "Drag" the screen we must negate the x
        translation.x *= -1.0;
        transform.translation += translation * projection.scale;
    }

    if mouse_scroll.delta.y != 0.0 {
        let window_middle: Vec2 = window.size() * 0.5;
        let mut zoom_amount = projection.scale * mouse_scroll.delta.y * SENSITIVITY;

        if mouse_scroll.unit == MouseScrollUnit::Pixel {
            zoom_amount /= 100.0
        }

        //We translate the camera so it zooms with the mouse in the middle
        let mut camera_translation = window
            .cursor_position()
            .map_or(window_middle, |v| v - window_middle);

        //We have to negate the y so that the zoom is towards the right position
        camera_translation.y *= -1.0;
        transform.translation += camera_translation.extend(0.0) * zoom_amount;
        projection.scale -= zoom_amount;
    }
}
