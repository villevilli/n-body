use bevy::{
    color::palettes::css::WHITE,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

const CMDLINE_FONT_SIZE: f32 = 16.0;
const CMDLINE_FONT: &str = "fonts/FiraMono-Regular.ttf";

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum CmdlineState {
    Open,
    Closed,
}

#[derive(Component)]
struct DevCommandlineTextMarker;

#[derive(Component)]
struct DevCommandlineMarker;

pub struct DevCommandline;

impl Plugin for DevCommandline {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(CmdlineState::Closed);
        app.add_systems(
            Update,
            (
                toggle_cmdline,
                update_cmdline.run_if(in_state(CmdlineState::Open)),
            ),
        );
        app.add_systems(OnEnter(CmdlineState::Open), enter_cmdline);
        app.add_systems(OnExit(CmdlineState::Open), exit_cmdline);
    }
}

fn toggle_cmdline(
    kb_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CmdlineState>>,
    mut next_state: ResMut<NextState<CmdlineState>>,
) {
    if kb_input.just_pressed(KeyCode::Backquote) {
        next_state.set(match state.get() {
            CmdlineState::Open => CmdlineState::Closed,
            CmdlineState::Closed => CmdlineState::Open,
        });
    }
}

fn enter_cmdline(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("> "),
            TextFont {
                font_size: CMDLINE_FONT_SIZE,
                font: assets.load(CMDLINE_FONT),
                ..Default::default()
            },
            TextColor(WHITE.into()),
            TextLayout {
                justify: JustifyText::Left,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            DevCommandlineMarker,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: CMDLINE_FONT_SIZE,
                font: assets.load(CMDLINE_FONT),
                ..Default::default()
            },
            TextColor(WHITE.into()),
            DevCommandlineTextMarker,
        ));
}

fn exit_cmdline(
    mut commands: Commands,
    cmdline_entity_query: Query<Entity, With<DevCommandlineMarker>>,
) {
    commands
        .entity(cmdline_entity_query.single())
        .despawn_recursive();
}

fn update_cmdline(
    mut ev_kb_input: EventReader<KeyboardInput>,
    mut cmdline_query: Query<&mut TextSpan, With<DevCommandlineTextMarker>>,
) {
    let mut text = cmdline_query.single_mut();

    for event in ev_kb_input.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        if event.key_code == KeyCode::Backquote {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                println!("Command: {}", text.0);
                text.0.clear();
            }
            Key::Backspace => {
                text.0.pop();
            }
            Key::Space => {
                text.0.push(' ');
            }
            Key::Character(c) => text.0.push_str(&c),
            _ => {}
        }
    }
}
