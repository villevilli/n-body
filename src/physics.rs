use bevy::math::bounding::BoundingCircle;
use bevy::{color::palettes::css::LIGHT_BLUE, prelude::*};

const GRAVITATIONAL_CONSTANT: f32 = 6740.0;

pub struct PhysicsPlugin<S: States> {
    pub running_state: S,
}

impl<S: States> Plugin for PhysicsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            calculate_physics.run_if(in_state(self.running_state.clone())),
        );
        app.add_systems(Update, move_physics_entities_visual);
    }
}

#[derive(Component, Debug)]
pub struct Collider(pub BoundingCircle);

#[derive(Component, Clone, Copy)]
pub struct PhysicsTransform {
    pub(crate) location: Vec2,
}

impl PhysicsTransform {
    pub fn move_by_velocity(&mut self, velocity: Vec2, delta: f32) {
        self.location += velocity * delta
    }
}

#[derive(Component, Clone, Copy)]
pub struct PhysicsMaterial {
    pub mass: f32,
}

#[derive(Component, Clone, Copy, Default)]
pub struct PhysicsVelocity {
    pub(crate) velocity: Vec2,
    acceleration: Vec2,
}

impl PhysicsVelocity {
    pub fn new(velocity: Vec2) -> Self {
        Self {
            velocity,
            ..Default::default()
        }
    }

    fn add_acceleration_from_force(&mut self, mass: f32, force: Vec2, delta: f32) {
        self.acceleration += force * mass.powi(-1) * delta
    }

    fn reset_acceleration(&mut self) {
        self.acceleration = Vec2::ZERO;
    }

    fn apply_acceleration(&mut self, delta: f32) {
        self.velocity += self.acceleration * delta
    }
}

fn calculate_physics(
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
    query.iter_mut().for_each(|(_, t, v)| {
        if let Some(mut v) = v {
            v.apply_acceleration(delta);
            gizmos.arrow_2d(t.location, t.location + v.acceleration, LIGHT_BLUE);

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

fn move_physics_entities_visual(
    mut query: Query<(&mut Transform, &PhysicsTransform, Option<&mut Collider>)>,
) {
    for (mut transform, physics_transform, mut collider) in query.iter_mut() {
        transform.translation.x = physics_transform.location.x;
        transform.translation.y = physics_transform.location.y;

        if let Some(collider) = collider.as_mut() {
            collider.0.center = physics_transform.location;
        }
    }
}

fn calculate_gravity_force(mass1: f32, mass2: f32, distance: f32) -> f32 {
    GRAVITATIONAL_CONSTANT * ((mass1 * mass2) / distance.powi(2))
}
