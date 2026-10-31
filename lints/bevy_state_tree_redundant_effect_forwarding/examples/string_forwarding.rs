#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Command {
    action: String,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("game.confirm", |_, _| {
            Ok(Command {
                action: "confirm".into(),
            })
        })
        .effect("game.back", |_, _| {
            let action = String::from("back");
            let message = Command { action };
            Ok(message)
        });
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        let action = command.action.as_str();
        match action {
            "confirm" => {}
            "back" => {}
            _ => {}
        }
    }
}

fn main() {}
