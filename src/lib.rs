pub mod mouse_camera_control;
pub mod physics;
pub mod world_constructor;

use bevy::{
    color::palettes::css::{BLUE, GREEN, WHITE},
    math::vec2,
    prelude::*,
};
use mouse_camera_control::MouseCameraControl;
use physics::{
    handle_physics, move_physics_entities_visual, PhysicsMaterial, PhysicsTransform,
    PhysicsVelocity,
};

pub struct NBodyPlatformer;

#[allow(dead_code)]
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SimulationState {
    Paused,
    Playing,
    Editing,
}

impl Plugin for NBodyPlatformer {
    fn build(&self, app: &mut App) {
        app.insert_state(SimulationState::Paused);
        app.add_plugins(MouseCameraControl {
            state: SimulationState::Editing,
        });
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (move_physics_entities_visual, keyboard_state_changer),
        );
        app.add_systems(StateTransition, print_state_on_change::<SimulationState>);
        app.add_systems(
            FixedUpdate,
            handle_physics.run_if(in_state(SimulationState::Playing)),
        );
    }
}

fn keyboard_state_changer(
    keys: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        use SimulationState::*;

        next_sim_state.set(match sim_state.get() {
            Paused => Playing,
            Playing => Paused,
            Editing => Editing,
        });
    }

    if keys.just_pressed(KeyCode::KeyE) {
        use SimulationState::*;

        next_sim_state.set(match sim_state.get() {
            Paused => Editing,
            Playing => Editing,
            Editing => Paused,
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PhysicsMaterial { mass: 24000.0 },
        PhysicsTransform {
            location: Vec2::new(30.0, 20.0),
        },
        PhysicsVelocity::default(),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
    ));

    commands.spawn((
        PhysicsMaterial { mass: 12.0 },
        PhysicsTransform {
            location: Vec2::new(500.0, 20.0),
        },
        PhysicsVelocity {
            velocity: (vec2(0.0, 20.0)),
            acceleration: vec2(0.0, 0.0),
        },
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
    ));

    commands.spawn((
        PhysicsMaterial { mass: 16.0 },
        PhysicsTransform {
            location: Vec2::new(-310.0, 20.0),
        },
        PhysicsVelocity {
            velocity: (vec2(0.0, 25.0)),
            acceleration: vec2(0.0, 0.0),
        },
        Mesh2d(meshes.add(Circle::new(2.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    println!("Simulation Set Up")
}

fn print_state_on_change<S>(mut state_change_ev: EventReader<StateTransitionEvent<S>>)
where
    S: States,
{
    if let Some(state_change) = state_change_ev.read().next() {
        println!("Entering state: {:?}", state_change.entered)
    }
}
