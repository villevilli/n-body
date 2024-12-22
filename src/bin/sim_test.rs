use bevy::color::palettes::css::*;
use bevy::{math::vec2, prelude::*};
use n_body_platformer::print_state_on_change;
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
            mass: 4200.0,
            position: vec2(0.0, 0.0),
            velocity: Some(vec2(0.0, 0.0)),
            color: GREEN.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 4200.0,
            position: vec2(200.0, 0.0),
            velocity: Some(vec2(0.0, 0.0)),
            color: WHITE_SMOKE.into(),
            ..Default::default()
        });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            MouseCameraControl {
                running_state: AlwaysOn,
            },
            PhysicsPlugin {
                running_state: SimulationState::Running,
            },
            LevelBuilderPlugin(level),
        ))
        .insert_state(AlwaysOn)
        .insert_state(SimulationState::Paused)
        .add_systems(
            Update,
            (
                keyboard_state_changer,
                print_state_on_change::<SimulationState>,
            ),
        )
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
