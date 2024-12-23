use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use n_body_platformer::commands::DevCommandline;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DevCommandline)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .run();
}

fn ui_system(mut context: EguiContexts) {
    egui::Window::new("Hello Gui").show(context.ctx_mut(), |ui| ui.label("World"));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
