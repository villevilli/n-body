use std::collections::VecDeque;

use bevy::prelude::*;

const TRAIL_LENGTH: usize = 300;

#[derive(Component, Default)]
pub struct Trail(VecDeque<Vec2>);

#[derive(Resource)]
pub struct TrailUpdateConfig(pub Timer);

impl Trail {
    fn draw_gizmo(&self, gizmos: &mut Gizmos) {
        let domain = Interval::new(0.0, TRAIL_LENGTH as f32).expect("Interval is invalid");

        let Ok(curve) = SampleAutoCurve::new(domain, self.0.clone()) else {
            return;
        };

        gizmos.curve_gradient_2d(
            curve,
            (0..TRAIL_LENGTH)
                .map(|x| x as f32)
                .map(|x| (x, Color::WHITE.with_alpha(-(x / TRAIL_LENGTH as f32) + 1.0))),
        );
    }

    fn add_to_trail(&mut self, point: Vec2) {
        self.0.push_front(point);
        if self.0.len() > TRAIL_LENGTH {
            self.0.pop_back();
        }
    }
}

pub fn update_trail(
    time: Res<Time>,
    mut trails: Query<(&mut Trail, &GlobalTransform)>,
    mut trail_update_config: ResMut<TrailUpdateConfig>,
) {
    trail_update_config.0.tick(time.delta());

    if !trail_update_config.0.finished() {
        return;
    }

    for (mut trail, transform) in trails.iter_mut() {
        trail.add_to_trail(transform.translation().truncate());
    }
}

pub fn draw_trail(mut gizmos: Gizmos, trails: Query<&Trail>) {
    for trail in trails.iter() {
        trail.draw_gizmo(&mut gizmos);
    }
}
