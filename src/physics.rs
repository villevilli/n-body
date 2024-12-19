#![allow(unused)]
use bevy::{color::palettes::css::RED, ecs::query, prelude::*};

const GRAVITATIONAL_CONSTANT: f32 = 60.674;

#[derive(Component, Clone, Copy)]
pub(crate) struct PhysicsTransform {
    pub(crate) location: Vec2,
}

impl PhysicsTransform {
    pub fn move_by_velocity(&mut self, velocity: Vec2, delta: f32) {
        self.location += velocity * delta
    }
}

#[derive(Component, Clone, Copy)]
pub(crate) struct PhysicsMaterial {
    pub(crate) mass: f32,
}

#[derive(Component, Clone, Copy, Default)]
pub(crate) struct PhysicsVelocity {
    pub(crate) velocity: Vec2,
    pub(crate) acceleration: Vec2,
}

struct CircleRender {
    size: f32,
    color: Color,
}

impl PhysicsVelocity {
    pub fn add_acceleration_from_force(&mut self, mass: f32, force: Vec2, delta: f32) {
        self.acceleration += force * mass.powi(-1) * delta
    }

    pub fn reset_acceleration(&mut self) {
        self.acceleration = Vec2::ZERO;
    }

    pub fn apply_acceleration(&mut self, delta: f32) {
        self.velocity += self.acceleration * delta
    }
}

pub(crate) fn handle_physics(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(
        &PhysicsMaterial,
        &mut PhysicsTransform,
        Option<&mut PhysicsVelocity>,
    )>,
) {
    let delta = time.delta_secs();
    let mut combinations = query.iter_combinations_mut();

    //Calculate and apply gravitational force on all applicable entities
    while let Some([object1, object2]) = combinations.fetch_next() {
        let direction = (object2.1.location - object1.1.location).normalize();

        let force = calculate_gravity_force(
            object1.0.mass,
            object2.0.mass,
            object1.1.location.distance(object2.1.location),
        ) * direction;

        if let Some(mut physicsvelocity) = object1.2 {
            physicsvelocity.add_acceleration_from_force(object1.0.mass, force, delta);
        }

        if let Some(mut physicsvelocity) = object2.2 {
            physicsvelocity.add_acceleration_from_force(object2.0.mass, -force, delta);
        }
    }

    //Calculate acceleration from force for entities
    query.iter_mut().for_each(|(_, t, mut v)| {
        if let Some(mut v) = v {
            v.apply_acceleration(delta);
            gizmos.arrow_2d(t.location, t.location + v.acceleration, RED);

            //Resets acceleration so we dont accumulate more next frame
            v.reset_acceleration();
        }
    });

    //Move entities by their velocity
    query.iter_mut().for_each(|(_, mut t, v)| {
        if let Some(v) = v {
            t.move_by_velocity(v.velocity, delta);
        }
    });
}

pub(crate) fn move_physics_entities_visual(mut query: Query<(&mut Transform, &PhysicsTransform)>) {
    for (mut transform, physics_transform) in &mut query {
        transform.translation.x = physics_transform.location.x;
        transform.translation.y = physics_transform.location.y;
    }
}

fn calculate_gravity_force(mass1: f32, mass2: f32, distance: f32) -> f32 {
    GRAVITATIONAL_CONSTANT * ((mass1 * mass2) / distance)
}
