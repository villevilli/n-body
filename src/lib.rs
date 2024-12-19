pub mod physics;
pub mod world_constructor;

use bevy::{
    color::palettes::css::{BLUE, GREEN, WHITE},
    math::vec2,
    prelude::*,
};
use physics::{
    handle_physics, move_physics_entities_visual, PhysicsMaterial, PhysicsTransform,
    PhysicsVelocity,
};

pub struct NBodyPlatformer;

impl Plugin for NBodyPlatformer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_physics_entities_visual);
        app.add_systems(FixedUpdate, handle_physics);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
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
            velocity: (vec2(0.0, 80.0)),
            acceleration: vec2(0.0, 0.0),
        },
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
    ));

    commands.spawn((
        PhysicsMaterial { mass: 0.4 },
        PhysicsTransform {
            location: Vec2::new(510.0, 20.0),
        },
        PhysicsVelocity {
            velocity: (vec2(0.0, 85.0)),
            acceleration: vec2(0.0, 0.0),
        },
        Mesh2d(meshes.add(Circle::new(2.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    println!("Simulation Set Up")
}
