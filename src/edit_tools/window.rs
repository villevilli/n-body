use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Component, Default)]
pub struct OpenWindow(bool);

pub fn detect_clicks(mut clicks: EventReader<Pointer<Click>>, mut commands: Commands) {
    for click in clicks.read() {
        commands
            .entity(click.target)
            .entry::<OpenWindow>()
            .or_default()
            .and_modify(|mut is_open| is_open.0 = !is_open.0);
    }
}

pub fn edit_windows(
    mut context: EguiContexts,
    window_object_query: Query<(Entity, &Transform, &OpenWindow)>,
) {
    for (entity, transform, open_window) in window_object_query.iter() {
        if !open_window.0 {
            continue;
        }

        egui::Window::new(format!("Planet Editor {}", entity.index())).show(
            context.ctx_mut(),
            |ui| {
                ui.label("Hello World!");
            },
        );
    }
}
