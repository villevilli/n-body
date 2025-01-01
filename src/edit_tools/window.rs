use bevy::prelude::*;
use bevy_egui::{
    egui::{self, DragValue, Ui},
    EguiContexts,
};

use crate::physics::{PhysicsMaterial, PhysicsTransform, PhysicsVelocity};

#[derive(Component, Default)]
pub struct OpenWindow {
    is_open: bool,
    just_changed: bool,
}

impl OpenWindow {
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

pub(super) fn detect_clicks(mut clicks: EventReader<Pointer<Click>>, mut commands: Commands) {
    for click in clicks.read() {
        commands
            .entity(click.target)
            .entry::<OpenWindow>()
            .or_default()
            .and_modify(|mut is_open| is_open.toggle());
    }
}

pub(super) fn edit_windows(
    mut commands: Commands,
    mut context: EguiContexts,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window_object_query: Query<(
        Entity,
        &mut OpenWindow,
        Option<&mut MeshMaterial2d<ColorMaterial>>,
        Option<&mut PhysicsTransform>,
        Option<&mut PhysicsVelocity>,
        Option<&mut PhysicsMaterial>,
    )>,
    window_query: Query<&Window>,
) {
    let cursor_pos = window_query.single().cursor_position();

    for (
        entity,
        mut open_window,
        color_material,
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
            .show(context.ctx_mut(), |ui| {
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
                    }
                    if ui.button("Despawn Entity").clicked() {
                        commands.entity(entity).despawn();
                    }
                })
            });
    }
}

fn material_color_editor_row(ui: &mut Ui, material: &mut ColorMaterial) {
    let mut color = material.color.to_linear().to_f32_array();

    ui.label("Planet Color");
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

fn vec2_editor(ui: &mut Ui, vec2: &mut Vec2) {
    ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut vec2.x).prefix("x: "));
        ui.add(DragValue::new(&mut vec2.y).prefix("y: "));
    });
}
