use crate::physics::{PhysicsMaterial, PhysicsTransform, PhysicsVelocity};
use bevy::prelude::*;
use ops::FloatPow;
use std::f32::consts::PI;

const PLANET_DENSITY: f32 = 1.0;

#[derive(Bundle, Clone)]
struct DynamicPlanet {
    physics_material: PhysicsMaterial,
    transform: PhysicsTransform,
    velocity: PhysicsVelocity,
    #[bundle(ignore)]
    style: PlanetStyle,
}

#[derive(Bundle)]
struct StaticPlanet {
    physics_material: PhysicsMaterial,
    transform: PhysicsTransform,
    #[bundle(ignore)]
    style: PlanetStyle,
}

#[derive(Default, Clone)]
struct PlanetStyle {
    radius: f32,
    color: Color,
}

enum SomePlanet {
    DynamicPlanet(DynamicPlanet),
    StaticPlanet(StaticPlanet),
}

///If not filled radius is derived from mass
///
///If given a none velocity the planet will be static
///
///If colour is none it will be picked at random
///
#[derive(Default, Clone)]
pub struct PlanetBuilder {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Option<Vec2>,
    pub radius: Option<f32>,
    pub color: Color,
}

impl PlanetBuilder {
    /// ## Panics
    /// Panics if attempted to build when mass or radius are negative
    fn build(self) -> SomePlanet {
        assert!(!self.mass.is_sign_negative());

        if let Some(r) = self.radius {
            assert!(!r.is_sign_negative())
        }

        let physics_material = PhysicsMaterial { mass: self.mass };
        let transform = PhysicsTransform {
            location: self.position,
        };

        let style = PlanetStyle {
            radius: self
                .radius
                .unwrap_or_else(move || (self.mass.sqrt() / PI.sqrt()) / PLANET_DENSITY),
            color: self.color,
        };

        match self.velocity {
            Some(velocity) => SomePlanet::DynamicPlanet(DynamicPlanet {
                physics_material,
                transform,
                style,
                velocity: PhysicsVelocity::new(velocity),
            }),
            None => SomePlanet::StaticPlanet(StaticPlanet {
                physics_material,
                transform,
                style,
            }),
        }
    }
}

#[derive(Default, Resource, Clone)]
pub struct LevelBuilder(Vec<PlanetBuilder>);

pub struct LevelBuilderPlugin(pub LevelBuilder);

impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());
        app.add_systems(Startup, construct_level_system);
    }
}

impl LevelBuilder {
    pub fn add_planet(mut self, planet: PlanetBuilder) -> Self {
        self.0.push(planet);
        self
    }
}
fn construct_level_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<LevelBuilder>,
) {
    for planet in level.0.drain(..) {
        match planet.build() {
            SomePlanet::DynamicPlanet(dynamic_planet) => {
                let style: PlanetStyle = dynamic_planet.style.clone();

                commands.spawn((
                    dynamic_planet,
                    Mesh2d(meshes.add(Circle::new(style.radius))),
                    MeshMaterial2d(materials.add(style.color)),
                ));
            }
            SomePlanet::StaticPlanet(static_planet) => {
                let style: PlanetStyle = static_planet.style.clone();

                commands.spawn((
                    static_planet,
                    Mesh2d(meshes.add(Circle::new(style.radius))),
                    MeshMaterial2d(materials.add(style.color)),
                ));
            }
        }
    }
}
