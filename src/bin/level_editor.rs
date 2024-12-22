#![feature(never_type)]

use bevy::prelude::*;
use n_body_platformer::{mouse_camera_control::MouseCameraControl, physics::PhysicsPlugin};

#[derive(States, Debug, PartialEq, Eq, Clone, Hash)]
enum SimulationState {
    Running,
    Paused,
}

#[derive(States, Debug, PartialEq, Eq, Clone, Hash, Default)]
struct AlwaysOn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            MouseCameraControl {
                running_state: AlwaysOn,
                ..Default::default()
            },
            PhysicsPlugin {
                running_state: SimulationState::Paused,
            },
        ))
        .insert_state(AlwaysOn)
        .insert_state(SimulationState::Paused)
        .add_systems(Update, keyboard_state_changer)
        .run();
}

fn keyboard_state_changer(
    keys: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        use SimulationState::*;

        next_sim_state.set(match sim_state.get() {
            Paused => Running,
            Running => Paused,
        });
    }
}
