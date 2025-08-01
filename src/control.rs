use bevy::ecs::{component::Component, system::Commands};

#[derive(Component)]
pub struct DropButton();


#[derive(Component)]
pub struct PauseButton();


#[derive(Component)]
pub struct AudioButton();


#[derive(Component)]
pub struct ReplayButton();


#[derive(Component)]
pub struct RotateButton();


#[derive(Component)]
pub struct LeftButton();


#[derive(Component)]
pub struct RightButton();

#[derive(Component)]
pub struct DownButton();




pub fn control_setup(commands: Commands) {}