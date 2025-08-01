mod brick_node;
mod control;
mod decorate;
mod board;
mod state;
mod game_data;

use bevy::prelude::*;

use state::GameSate;

use crate::{control::control_setup, decorate::decorate_setup};

fn main() {
    App::new().add_plugins(DefaultPlugins)
    .init_state::<GameSate>()
    .add_systems(Startup, (control_setup, decorate_setup))
    .run();
}
