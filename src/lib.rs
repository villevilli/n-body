#![feature(let_chains)]

pub mod commands;
pub mod edit_tools;
pub mod graphics;
pub mod level_builder;
pub mod mouse_camera_control;
pub mod physics;

use bevy::prelude::*;

pub fn print_state_on_change<S>(mut state_change_ev: EventReader<StateTransitionEvent<S>>)
where
    S: States,
{
    if let Some(state_change) = state_change_ev.read().next() {
        println!("Entering state: {:?}", state_change.entered)
    }
}
