#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
enum Command {
    Start,
    Pause,
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("arcade.start", |_, _| Ok(Command::Start))
        .effect("arcade.pause", |_, _| Ok(Command::Pause));
}

fn main() {}
