use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use n_body_platformer::commands::{
    DevCommandlinePlugin,
    command_parser::{DevCommand, DevCommandList},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui_system);

    let commands = DevCommandList::new()
        .add_command(DevCommand::<String>::new(
            "info",
            IntoSystem::into_system(info_cmd),
            app.world_mut(),
        ))
        .add_command(DevCommand::new(
            "infotwo",
            IntoSystem::into_system(info_two),
            app.world_mut(),
        ));

    app.add_plugins(DevCommandlinePlugin)
        .insert_resource(commands)
        .run();
}

fn ui_system(mut context: EguiContexts) {
    egui::Window::new("Hello Gui").show(context.ctx_mut().unwrap(), |ui| ui.label("World"));
}

fn info_cmd(text: In<String>) {
    info!("{}", text.0);
}

fn info_two(text: In<String>) {
    info!("info_two: {}", text.0)
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
