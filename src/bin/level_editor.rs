use bevy::color::palettes::css::*;
use bevy::{math::vec2, prelude::*};
use bevy_egui::{
    egui::{self, DragValue},
    EguiContexts, EguiPlugin,
};
use n_body_platformer::edit_tools::EditTools;
use n_body_platformer::mouse_camera_control::MainCameraMarker;
use n_body_platformer::{
    level_builder::{LevelBuilder, LevelBuilderPlugin, PlanetBuilder},
    mouse_camera_control::MouseCameraControl,
    physics::PhysicsPlugin,
};

#[derive(States, Debug, PartialEq, Eq, Clone, Hash)]
enum SimulationState {
    Running,
    Paused,
}

#[derive(States, Debug, PartialEq, Eq, Clone, Hash, Default)]
struct AlwaysOn;

fn main() {
    let lb = LevelBuilder::default()
        .add_planet(PlanetBuilder {
            mass: 300.0,
            position: vec2(0.0, 0.0),
            velocity: Some(vec2(0.0, 0.0)),
            color: GREEN.into(),
            ..Default::default()
        })
        .add_planet(PlanetBuilder {
            mass: 3600.0,
            position: vec2(300.0, 0.0),
            velocity: Some(vec2(0.0, 200.0)),
            color: PINK.into(),
            ..Default::default()
        });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            MouseCameraControl {
                running_state: AlwaysOn,
                ..Default::default()
            },
            PhysicsPlugin {
                running_state: SimulationState::Running,
            },
            LevelBuilderPlugin(lb),
            EditTools::<MainCameraMarker> {
                main_camera_type: std::marker::PhantomData,
            },
            EguiPlugin,
        ))
        .add_systems(Update, (keyboard_state_changer, egui_window_test))
        .insert_state(AlwaysOn)
        .insert_state(SimulationState::Paused)
        .run();
}

#[derive(Default)]
struct GuiData {
    color: [f32; 3],
    velocity: Vec2,
    position: Vec2,
    mass: f32,
}

fn egui_window_test(mut context: EguiContexts, mut gui_data: Local<GuiData>) {
    let (mut vel_x, mut vel_y) = gui_data.velocity.into();
    let (mut pos_x, mut pos_y) = gui_data.position.into();

    egui::Window::new("Planet Editor")
        .resizable([false, false])
        .show(context.ctx_mut(), |ui| {
            egui::Grid::new("lol").num_columns(2).show(ui, |ui| {
                ui.label("Planet Color");
                ui.color_edit_button_rgb(&mut gui_data.color);

                ui.end_row();

                ui.label("Velocity");
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut vel_x).prefix("x: "));
                    ui.add(DragValue::new(&mut vel_y).prefix("y: "));
                });

                ui.end_row();

                ui.label("Position");
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut pos_x).prefix("x: "));
                    ui.add(DragValue::new(&mut pos_y).prefix("y: "));
                });
                ui.end_row();
                ui.label("Mass");
                ui.add(DragValue::new(&mut gui_data.mass));
            })
        });

    gui_data.velocity = (vel_x, vel_y).into();
    gui_data.position = (pos_x, pos_y).into();
}

fn keyboard_state_changer(
    keys: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        use SimulationState::*;

        next_sim_state.set(match sim_state.get() {
            Paused => Running,
            Running => Paused,
        });
    }
}
