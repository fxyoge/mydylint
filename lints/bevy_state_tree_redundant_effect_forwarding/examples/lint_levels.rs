#![allow(dead_code)]
#![deny(unfulfilled_lint_expectations)]

use bevy_ecs::prelude::*;
use state_tree::StateTreeConfig;

#[derive(Message)]
enum Command {
    Start,
    Pause,
}

fn configure(config: &mut StateTreeConfig<'_>) {
    config
        .effect("level.start", |_, _| Ok(Command::Start))
        .effect("level.pause", |_, _| Ok(Command::Pause));
}

#[cfg_attr(
    dylint_lib = "bevy_state_tree_redundant_effect_forwarding",
    allow(
        bevy_state_tree_redundant_effect_forwarding,
        reason = "exercise node-scoped lint levels"
    )
)]
fn allowed(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

#[cfg_attr(
    dylint_lib = "bevy_state_tree_redundant_effect_forwarding",
    expect(
        bevy_state_tree_redundant_effect_forwarding,
        reason = "exercise node-scoped expectations"
    )
)]
fn expected(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn forbidden(mut commands: MessageReader<Command>) {
    for command in commands.read() {
        match command {
            Command::Start => {}
            Command::Pause => {}
        }
    }
}

fn main() {}
