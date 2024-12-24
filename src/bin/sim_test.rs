use bevy::color::palettes::css::*;
use bevy::{math::vec2, prelude::*};
use n_body_platformer::commands::command_parser::{DevCommand, DevCommandList};
use n_body_platformer::commands::{CmdlineState, DevCommandlinePlugin};
use n_body_platformer::mouse_camera_control::CameraSettings;
use n_body_platformer::world_constructor::LevelBuilderPlugin;
use n_body_platformer::{
    mouse_camera_control::MouseCameraControl,
    physics::PhysicsPlugin,
    world_constructor::{LevelBuilder, PlanetBuilder},
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
        .insert_state(AlwaysOn);

    let dev_commands = DevCommandList::new().add_command(DevCommand::new(
        "setsimspeed",
        IntoSystem::into_system(set_sim_speed),
        app.world_mut(),
    ));

    app.insert_resource(dev_commands)
        .add_plugins(DevCommandlinePlugin);

    #[cfg(not(target_family = "wasm"))]
    app.insert_state(SimulationState::Paused);

    #[cfg(target_family = "wasm")]
    app.insert_state(SimulationState::Running);

    app.add_systems(Update, (keyboard_state_changer,)).run();
}

fn set_sim_speed(speed: In<f32>, mut time: ResMut<Time<Virtual>>) {
    info!("Sim speed multiplier set to: {}", speed.0);
    time.set_relative_speed(speed.0);
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
