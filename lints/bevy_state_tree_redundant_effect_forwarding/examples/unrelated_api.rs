#![allow(dead_code)]

use bevy_ecs::prelude::*;

#[derive(Message)]
enum Command {
    Start,
    Pause,
}

struct FakeConfig;

impl FakeConfig {
    fn effect(&mut self, _: &str, _: impl Fn() -> Command) -> &mut Self {
        self
    }
}

fn configure(config: &mut FakeConfig) {
    config
        .effect("arcade.start", || Command::Start)
        .effect("arcade.pause", || Command::Pause);
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
