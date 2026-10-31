#![allow(dead_code)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig as Config;

type Protocol<'a> = Config<'a>;

mod messages {
    use bevy_ecs::prelude::*;

    #[derive(Message)]
    pub enum Command {
        Start,
        Pause,
    }
}

use messages::Command as Renamed;
type Command = Renamed;

fn configure(config: &mut Protocol<'_>) {
    Protocol::effect(config, "arcade.start", |_, _| {
        let command = Command::Start;
        Ok(command)
    });
    Config::effect(config, "arcade.pause", |_, _| Ok(Renamed::Pause));
}

fn consume(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Renamed::Start => {}
            messages::Command::Pause => {}
        }
    }
}

fn main() {}
