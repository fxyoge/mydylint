#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Start;
#[derive(Message)]
struct Pause;

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("arcade.start", |_, _| Ok(Start))
        .effect("arcade.pause", |_, _| Ok(Pause));
}

fn first_reader(mut messages: MessageReader<Start>) {
    for _ in messages.read() {}
}

fn second_reader(mut messages: MessageReader<Start>) {
    for _ in messages.read() {}
}

fn main() {}
