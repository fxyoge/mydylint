#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Registered {
    action: String,
}

#[derive(Message)]
struct Unrelated {
    action: String,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("game.start", |_, _| {
            Ok(Registered {
                action: "start".into(),
            })
        })
        .effect("game.pause", |_, _| {
            Ok(Registered {
                action: "pause".into(),
            })
        });
}

fn consume(mut commands: MessageReader<Unrelated>) {
    for command in commands.read() {
        match command.action.as_str() {
            "start" => {}
            "pause" => {}
            _ => {}
        }
    }
}

fn main() {}
