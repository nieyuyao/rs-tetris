
use bevy::state::state::States;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameSate {
    #[default]
    Ready,
    Playing,
    Paused,
    GameOver,
}
