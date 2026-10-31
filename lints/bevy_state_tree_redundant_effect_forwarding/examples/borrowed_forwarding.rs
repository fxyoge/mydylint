#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Command {
    action: &'static str,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("game.confirm", |_, _| Ok(Command { action: "confirm" }))
        .effect("game.back", |_, _| Ok(Command { action: "back" }));
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command.action {
            "confirm" | "back" => {}
            _ => {}
        }
    }
}

fn main() {}
