#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Command {
    action: String,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config.effect("game.action", |_, args| {
        Ok(Command {
            action: args.take("action")?,
        })
    });
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command.action.as_str() {
            "quit" => {}
            "fullscreen" => {}
            _ => {}
        }
    }
}

fn main() {}
