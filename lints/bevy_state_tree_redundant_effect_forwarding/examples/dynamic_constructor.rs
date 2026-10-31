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
        .effect("arcade.start", |_, args| {
            if args.take::<bool>("pause")? {
                return Ok(Command::Pause);
            }
            Ok(Command::Start)
        })
        .effect("arcade.pause", |_, _| Ok(Command::Pause));
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
