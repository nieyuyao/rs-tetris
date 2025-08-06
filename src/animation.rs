use std::time::Duration;

use bevy::{ecs::{component::Component, system::{Res, ResMut, Single}}, prelude::{Deref, DerefMut}, sprite::Sprite, time::{Time, Timer}};

use crate::game_data::GameData;


#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub (crate) Timer);

pub fn play_ready_animation(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    query: Single<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>
) {
    let delta: std::time::Duration = time.delta();
    game_data.playing_ready_animation_duration = game_data.playing_ready_animation_duration.saturating_add(delta);

    let (mut indices, mut timer, mut sprite) = query.into_inner();

    if game_data.is_playing_dino_running_animation {
        if game_data.playing_ready_animation_duration.as_secs() > 6 {
            indices.first = 0;
            indices.last = 1;
            game_data.playing_ready_animation_duration = Duration::default();
            game_data.is_playing_dino_running_animation = false;
            timer.0.set_duration(Duration::from_secs_f32(1.));
        }
    } else {
        if game_data.playing_ready_animation_duration.as_secs() > 6 {
            indices.first = 2;
            indices.last = 3;
            game_data.playing_ready_animation_duration = Duration::default();
            game_data.is_playing_dino_running_animation = true;
            timer.0.set_duration(Duration::from_secs_f32(0.1));
        }
    }

    timer.tick(delta);

    if timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}