use bevy::color::palettes::css::*;
use bevy::{math::vec2, prelude::*};
use n_body_platformer::commands::command_parser::DevCommandList;
use n_body_platformer::commands::{CmdlineState, DevCommandlinePlugin};
use n_body_platformer::edit_tools::picking_backend_physics;
use n_body_platformer::level_builder::LevelBuilderPlugin;
use n_body_platformer::mouse_camera_control::{CameraSettings, MainCameraMarker};
use n_body_platformer::{
    level_builder::{LevelBuilder, PlanetBuilder},
    mouse_camera_control::MouseCameraControl,
    physics::PhysicsPlugin,
};

#[derive(States, Debug, PartialEq, Eq, Clone, Hash)]
enum SimulationState {
    Running,
    Paused,
}

#[derive(States, Debug, PartialEq, Eq, Clone, Hash)]
struct AlwaysOn;

fn main() {
    let level = LevelBuilder::default()
        .add_planet(PlanetBuilder {
            mass: 420000.0,
            position: vec2(0.0, 0.0),
            velocity: Some(vec2(0.0, 0.0)),
            color: ORANGE.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 4200.0,
            position: vec2(1600.0, 0.0),
            velocity: Some(vec2(0.0, 150.0)),
            color: GREEN.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 40.0,
            position: vec2(1500.0, 0.0),
            velocity: Some(vec2(0.0, 96.0)),
            color: WHITE_SMOKE.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 3700.0,
            position: vec2(600.0, 300.0),
            velocity: Some(vec2(-180.0, 230.0)),
            color: RED.into(),
            ..Default::default()
        });

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((
            MouseCameraControl {
                running_state: AlwaysOn,
                camera_settings: CameraSettings {
                    zoom: 3.0,
                    ..Default::default()
                },
            },
            PhysicsPlugin {
                running_state: SimulationState::Running,
            },
            LevelBuilderPlugin(level),
        ))
        .add_systems(Update, picking_backend_physics::<MainCameraMarker>)
        .add_systems(Update, read_clicks)
        .insert_state(AlwaysOn);

    let dev_commands = DevCommandList::new().add_default_commands(app.world_mut());

    app.insert_resource(dev_commands)
        .add_plugins(DevCommandlinePlugin);

    #[cfg(not(target_family = "wasm"))]
    app.insert_state(SimulationState::Paused);

    #[cfg(target_family = "wasm")]
    app.insert_state(SimulationState::Running);

    app.add_systems(Update, (keyboard_state_changer,)).run();
}

fn read_clicks(click: EventReader<Pointer<Click>>) {
    if click.is_empty() {
        return;
    }
    info!("Clicked Physics Object")
}

fn keyboard_state_changer(
    keys: Res<ButtonInput<KeyCode>>,
    cmdline_state: Res<State<CmdlineState>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if cmdline_state.get() == &CmdlineState::Open {
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        use SimulationState::*;

        next_sim_state.set(match sim_state.get() {
            Paused => Running,
            Running => Paused,
        });
    }
}
