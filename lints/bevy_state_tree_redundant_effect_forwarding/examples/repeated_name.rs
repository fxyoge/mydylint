#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
enum Command {
    Start,
    Pause,
}

fn alternative_a(config: &mut StateTreeConfig<'_>) {
    config.effect("arcade.action", |_, _| Ok(Command::Start));
}

fn alternative_b(config: &mut StateTreeConfig<'_>) {
    config.effect("arcade.action", |_, _| Ok(Command::Pause));
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn main() {}
