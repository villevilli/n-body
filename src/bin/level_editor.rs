use bevy::color::palettes::css::*;
use bevy::{math::vec2, prelude::*};
use bevy_egui::EguiPlugin;
use n_body_platformer::edit_tools::EditingToolsPlugin;
use n_body_platformer::mouse_camera_control::MainCameraMarker;
use n_body_platformer::{
    level_builder::{LevelBuilder, LevelBuilderPlugin, PlanetBuilder},
    mouse_camera_control::MouseCameraControl,
    physics::PhysicsPlugin,
};

#[derive(States, Debug, PartialEq, Eq, Clone, Hash)]
enum SimulationState {
    Running,
    Paused,
}

#[derive(States, Debug, PartialEq, Eq, Clone, Hash, Default)]
struct AlwaysOn;

fn main() {
    let lb = LevelBuilder::default()
        .add_planet(PlanetBuilder {
            mass: 300.0,
            position: vec2(0.0, 0.0),
            velocity: Some(vec2(0.0, 0.0)),
            color: GREEN.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 3600.0,
            position: vec2(300.0, 0.0),
            velocity: Some(vec2(0.0, 200.0)),
            color: PINK.into(),
            ..Default::default()
        });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            MouseCameraControl {
                running_state: AlwaysOn,
                ..Default::default()
            },
            PhysicsPlugin {
                running_state: SimulationState::Running,
            },
            LevelBuilderPlugin(lb),
            EditingToolsPlugin::<MainCameraMarker> {
                main_camera_type: std::marker::PhantomData,
            },
            EguiPlugin::default(),
        ))
        .add_systems(Update, keyboard_state_changer)
        .insert_state(AlwaysOn)
        .insert_state(SimulationState::Paused)
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
