pub mod window;
use std::marker::PhantomData;

use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume},
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
    },
    prelude::*,
};
use bevy_egui::{EguiPrimaryContextPass, egui::Ui};
use window::{
    CreateNewPlanet, create_planet_window, detect_planet_creation, edit_windows,
    toggle_editor_window,
};

use crate::physics::Collider;

/// This Plugin contains a picking backend for the physics objects and
/// custom egui editor for editing physics objects
///
/// ##Usage
/// T should be the main camera as it is used to compute the position of
/// mouse clicks within the game world
#[derive(Default)]
pub struct EditingToolsPlugin<T>
where
    T: Component,
{
    pub main_camera_type: PhantomData<T>,
}

impl<T> Plugin for EditingToolsPlugin<T>
where
    T: Component,
{
    fn build(&self, app: &mut App) {
        app.add_message::<CreateNewPlanet>();
        app.add_systems(
            Update,
            (picking_backend_physics::<T>, detect_planet_creation::<T>),
        );
        app.add_systems(EguiPrimaryContextPass, (edit_windows, create_planet_window));
        app.add_observer(toggle_editor_window);
    }
}

//TODO this should probably be independant of the edit tools

pub fn picking_backend_physics<MainCameraMarker>(
    camera_query: Query<(Entity, &Camera, &GlobalTransform), With<MainCameraMarker>>,
    pointer_locations: Query<(&PointerLocation, &PointerId)>,
    physics_objects: Query<(&Collider, Entity)>,
    mut ew_pointerhits: MessageWriter<PointerHits>,
) where
    MainCameraMarker: Component,
{
    let Ok((camera_entity, camera, camera_global_transform)) = camera_query.single() else {
        panic!("Missing camera to initialize picking backend")
    };

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
                ew_pointerhits.write(event);
            }
        }
    }
}

pub trait EditableComponent: Component {
    fn edit_ui(&mut self, ui: &mut Ui);
}
