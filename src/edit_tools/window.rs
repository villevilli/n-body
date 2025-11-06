use bevy::{ecs::component::Mutable, prelude::*};
use bevy_egui::{
    EguiContexts,
    egui::{self, DragValue, Grid, Ui},
};
use rand::random;

use crate::{
    edit_tools::EditableComponent,
    level_builder::{PlanetBuilder, calculate_radius},
    physics::{PhysicsMaterial, PhysicsTransform, PhysicsVelocity},
};

#[derive(Message)]
pub(super) struct CreateNewPlanet(Vec2);

#[derive(Component, Default)]
pub struct EditorWindow {
    is_open: bool,
    just_changed: bool,
}

impl EditorWindow {
    pub fn new(is_open: bool) -> Self {
        Self {
            is_open,
            just_changed: true,
        }
    }

    pub fn get(&self) -> bool {
        self.is_open
    }

    pub fn set(&mut self, is_open: bool) {
        self.is_open = is_open;
        self.just_changed = true;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.just_changed = true;
    }
}

pub(super) fn toggle_editor_window(click: On<Pointer<Click>>, mut commands: Commands) {
    let click = click.event_target();
    commands
        .entity(click)
        .entry::<EditorWindow>()
        .or_default()
        .and_modify(|mut is_open| is_open.toggle());
}

pub(super) fn detect_planet_creation<T>(
    mut create_planet_ew: EventWriter<CreateNewPlanet>,
    keys: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<T>>,
    window_query: Query<&Window>,
) where
    T: Component,
{
    if !keys.just_released(KeyCode::KeyN) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        panic!("No camera for determining user planet creation")
    };
    let cursor_pos = window_query
        .single()
        .map(|t| t.cursor_position().unwrap_or_default())
        .expect("One window should exist");

    let planet_pos = match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        Ok(x) => x,
        Err(e) => {
            error!("{:#?}", e);
            return;
        }
    };

    create_planet_ew.write(CreateNewPlanet(planet_pos));
}

#[allow(clippy::too_many_arguments)]
pub(super) fn create_planet_window(
    mut gizmos: Gizmos,
    mut create_planet_event: EventReader<CreateNewPlanet>,
    mut current_planet: Local<Option<PlanetBuilder>>,
    mut is_open: Local<bool>,
    mut context: EguiContexts,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window>,
) {
    let cursor_pos: Vec2 = window_query
        .single()
        .map(|t| t.cursor_position().unwrap_or_default())
        .expect("One window should exist");

    let mut window = egui::Window::new("Planet Creator");

    if let Some(planet_creation_event) = create_planet_event.read().next() {
        let planet_ref = current_planet.get_or_insert_default();

        planet_ref.position = planet_creation_event.0;
        planet_ref.color = Color::hsv(random::<f32>() * 360.0, 1.0, 1.0);
        *is_open = true;
        window = window.current_pos(cursor_pos.to_array());
    }

    let Some(current_planet) = &mut *current_planet else {
        return;
    };

    let mut should_close = false;

    //Draw gizmos for planet preview
    gizmos.circle_2d(
        current_planet.position,
        current_planet
            .radius
            .unwrap_or(calculate_radius(current_planet.mass)),
        current_planet.color.with_alpha(0.6),
    );

    window
        .resizable([false; 2])
        .collapsible(false)
        .open(&mut is_open)
        .show(
            context.ctx_mut().expect("Egui context should exist"),
            |ui| {
                Grid::new("lol").show(ui, |ui| {
                    ui.label("Planet Color");

                    let mut color = current_planet.color.to_linear().to_f32_array();

                    ui.color_edit_button_rgba_premultiplied(&mut color);

                    current_planet.color = Color::LinearRgba(LinearRgba::from_f32_array(color));
                    ui.end_row();

                    //Position
                    ui.label("Position");
                    vec2_editor(ui, &mut current_planet.position);
                    ui.end_row();

                    //Velocity
                    ui.label("Velocity");
                    vec2_editor(ui, current_planet.velocity.get_or_insert_default());
                    ui.end_row();

                    //Mass editor
                    ui.label("Mass");
                    ui.add(DragValue::new(&mut current_planet.mass));
                    ui.end_row();

                    //Create Button
                    if ui.button("Create Planet").clicked() {
                        current_planet.clone().build().build(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                        );

                        should_close = true;
                    }
                });
            },
        );

    if should_close {
        *is_open = false;
    }
}

macro_rules! edit_components {
    ($ui:expr, $entity:expr, $($edited_type:ty),+) => {
        $(
            try_edit_type::<$edited_type>($ui, $entity);
        )+
    };
}

pub(super) fn edit_windows(
    mut context: EguiContexts,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut window_object_query: Query<(
        Entity,
        &mut EditorWindow,
        Option<&mut MeshMaterial2d<ColorMaterial>>,
        Option<&mut Mesh2d>,
        Option<&mut PhysicsTransform>,
        Option<&mut PhysicsVelocity>,
        Option<&mut PhysicsMaterial>,
    )>,
    window_query: Query<&Window>,
    mut commands: Commands,
) {
    let cursor_pos = window_query
        .single()
        .expect("One window should exist")
        .cursor_position();

    for (
        entity,
        mut open_window,
        color_material,
        mesh_handle,
        physics_transform,
        physics_velocity,
        physics_material,
    ) in window_object_query.iter_mut()
    {
        let mut window = egui::Window::new(format!("Planet Editor {}", entity.index()));

        if let Some(cursor_pos) = cursor_pos
            && open_window.just_changed
        {
            window = window.current_pos(cursor_pos.to_array());
            open_window.just_changed = false;
        }

        window
            .resizable([false; 2])
            .open(&mut open_window.is_open)
            .show(context.ctx_mut().unwrap(), |ui| {
                egui::Grid::new("lol").show(ui, |ui| {
                    if let Some(Some(material)) =
                        color_material.map(|x| materials.get_mut(x.as_ref()))
                    {
                        material_color_editor_row(ui, material);
                    }
                    if let Some(mut physics_transform) = physics_transform {
                        position_editor_row(ui, physics_transform.as_mut())
                    }
                    if let Some(mut physics_velocity) = physics_velocity {
                        velocity_editor(ui, physics_velocity.as_mut());
                    }
                    if let Some(mut physics_material) = physics_material {
                        mass_editor(ui, physics_material.as_mut());

                        if let Some(mesh_handle) = mesh_handle {
                            meshes.insert(
                                mesh_handle.id(),
                                Circle::new(calculate_radius(physics_material.mass))
                                    .mesh()
                                    .build(),
                            );
                        }
                    }
                    if ui.button("Despawn Entity").clicked() {
                        commands.entity(entity).despawn();
                    }
                });
            });
    }
}

fn try_edit_type<EditedType>(ui: &mut Ui, entity: &mut EntityMut)
where
    EditedType: EditableComponent,
    EditedType: Component<Mutability = Mutable>,
{
    if let Some(mut to_be_edited) = entity.get_mut::<EditedType>() {
        to_be_edited.edit_ui(ui)
    }
}

fn material_color_editor_row(ui: &mut Ui, material: &mut ColorMaterial) {
    let mut color = material.color.to_linear().to_f32_array();

    ui.label("Color");
    ui.color_edit_button_rgba_premultiplied(&mut color);
    ui.end_row();

    material.color = Color::LinearRgba(LinearRgba::from_f32_array(color))
}

fn position_editor_row(ui: &mut Ui, transform: &mut PhysicsTransform) {
    ui.label("Position: ");
    vec2_editor(ui, &mut transform.location);
    ui.end_row();
}

fn velocity_editor(ui: &mut Ui, velocity: &mut PhysicsVelocity) {
    ui.label("Velocity: ");
    vec2_editor(ui, &mut velocity.velocity);
    ui.end_row();
}

fn mass_editor(ui: &mut Ui, mass: &mut PhysicsMaterial) {
    ui.label("mass: ");
    ui.add(DragValue::new(&mut mass.mass));
    ui.end_row();
}

pub fn vec2_editor(ui: &mut Ui, vec2: &mut Vec2) {
    ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut vec2.x).prefix("x: "));
        ui.add(DragValue::new(&mut vec2.y).prefix("y: "));
    });
}
