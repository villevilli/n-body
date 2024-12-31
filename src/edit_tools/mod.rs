pub mod window;
use std::marker::{PhantomData, PhantomPinned};

use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume},
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
    },
    prelude::*,
};
use window::{detect_clicks, edit_windows};

use crate::physics::Collider;

pub struct EditTools<T>
where
    T: Component,
{
    pub main_camera_type: PhantomData<T>,
}

impl<T> Plugin for EditTools<T>
where
    T: Component,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (picking_backend_physics::<T>, detect_clicks));
        app.add_systems(Update, edit_windows);
    }
}

///T is marker component for the main camera
pub fn picking_backend_physics<T>(
    camera_query: Query<(Entity, &Camera, &GlobalTransform), With<T>>,
    pointer_locations: Query<(&PointerLocation, &PointerId)>,
    physics_objects: Query<(&Collider, Entity)>,
    mut ew_pointerhits: EventWriter<PointerHits>,
) where
    T: Component,
{
    let (camera_entity, camera, camera_global_transform) = camera_query.single();

    for (pointer_location, pointer_id) in pointer_locations.iter() {
        if let Some(pointer_location) = pointer_location.location() {
            let mut event = PointerHits::new(*pointer_id, Vec::new(), 1.0);

            for (collider, entity) in physics_objects.iter() {
                match collider {
                    Collider(bounding_circle) => {
                        let world_pointer_location = camera
                            .viewport_to_world_2d(
                                camera_global_transform,
                                pointer_location.position,
                            )
                            .unwrap();

                        if bounding_circle
                            .intersects(&BoundingCircle::new(world_pointer_location, 1.0))
                        {
                            event
                                .picks
                                .push((entity, HitData::new(camera_entity, 0.0, None, None)));
                        }
                    }
                }
            }
            if !event.picks.is_empty() {
                ew_pointerhits.send(event);
            }
        }
    }
}
