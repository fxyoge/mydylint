#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
enum Command {
    Start,
    Pause,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("arcade.start", |_, _| Ok(Command::Start))
        .effect("arcade.pause", |_, _| Ok(Command::Pause));
}

fn enumerated(mut commands: MessageReader<Command>) {
    let pending = commands.read();
    for (index, command) in pending.enumerate() {
        let _first = index == 0;
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn with_id(mut commands: MessageReader<Command>) {
    for (command, _) in commands.read_with_id() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn main() {}
