use std::time::Duration;

use bevy::{
    color::Srgba, ecs::{
        component::Component,
        query::{With, Without},
        system::{Query, Res, ResMut, Single},
    }, hierarchy::Children, prelude::{Deref, DerefMut}, sprite::Sprite, time::{Time, Timer}
};
use bevy_prototype_lyon::draw::{Fill, Stroke};

use crate::{
    board::{clean_board_lines, BoardBrickNode, FallingBrickNode},
    brick::BrickNode,
    game_data::{EraseAnimationStep, GameData},
};

#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub(crate) Timer);

pub fn play_ready_animation(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    query: Single<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    let delta: std::time::Duration = time.delta();
    game_data.ready_animation_duration = game_data.ready_animation_duration.saturating_add(delta);

    let (mut indices, mut timer, mut sprite) = query.into_inner();

    if game_data.is_playing_dino_running_animation {
        if game_data.ready_animation_duration.as_secs() > 6 {
            indices.first = 0;
            indices.last = 1;
            game_data.ready_animation_duration = Duration::default();
            game_data.is_playing_dino_running_animation = false;
            timer.0.set_duration(Duration::from_secs_f32(1.));
        }
    } else {
        if game_data.ready_animation_duration.as_secs() > 6 {
            indices.first = 2;
            indices.last = 3;
            game_data.ready_animation_duration = Duration::default();
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

pub fn play_erase_animation(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut board_brick_nodes_query: Query<(&mut Children, &BrickNode), (With<BoardBrickNode>, Without<FallingBrickNode>)>,
    mut fill_query: Query<&mut Fill>,
    mut stroke_query: Query<&mut Stroke>,
) {
    if game_data.erase_animation_step == EraseAnimationStep::Playing {
        let delta: std::time::Duration = time.delta();
        game_data.erase_animation_duration = game_data.erase_animation_duration.saturating_add(delta);
        let (start, lines) = game_data.clean_lines;

        game_data.erase_animation_timer.tick(delta);
        if game_data.erase_animation_timer.finished() {
            game_data.erase_animation_index += 1;
            board_brick_nodes_query
                .iter()
                .filter(|(_, node)| node.1 >= (start as i8) && node.1 < (start + lines) as i8).for_each(|(children,..)| {
                    for child in children.iter() {
                                if let Ok(mut fill) = fill_query.get_mut(*child) {
                                    fill.color = if game_data.erase_animation_index % 2 == 0 {
                                        Srgba::hex("#000000").unwrap().into()
                                    } else {
                                        Srgba::hex("#879372").unwrap().into()
                                    }
                                }
                                if let Ok(mut stroke) = stroke_query.get_mut(*child) {
                                    stroke.color = if game_data.erase_animation_index % 2 == 0 {
                                        Srgba::hex("#000000").unwrap().into()
                                    } else {
                                        Srgba::hex("#879372").unwrap().into()
                                    }
                                }
                            }
                });
        }
        
        if game_data.erase_animation_duration.as_secs() >= 2 {
            clean_board_lines(
                &mut board_brick_nodes_query,
                &mut game_data,
                &mut fill_query,
                &mut stroke_query,
            );
            game_data.erase_animation_step = EraseAnimationStep::End;
        }
    }
}
