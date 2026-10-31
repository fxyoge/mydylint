#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
struct Command {
    action: String,
    value: String,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("text.a", |_, args| {
            Ok(Command {
                action: "a".into(),
                value: args.take("value")?,
            })
        })
        .effect("text.b", |_, args| {
            Ok(Command {
                action: "b".into(),
                value: args.take("value")?,
            })
        });
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command.value.as_str() {
            "a" => {}
            "b" => {}
            _ => {}
        }
        let _ = command.action.len();
    }
}

fn main() {}
