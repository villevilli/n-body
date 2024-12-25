use bevy::{ecs::system::SystemId, prelude::*};
use radix_trie::Trie;
use std::{error::Error, str::FromStr};

#[derive(Resource)]
pub struct DevCommandList(pub(super) Trie<String, Box<dyn Runnable + Sync + Send>>);

impl DevCommandList {
    pub fn new() -> Self {
        Self(Trie::new())
    }

    pub fn add_command<I>(mut self, dev_command: DevCommand<I>) -> Self
    where
        I: FromStr + Send + Sync,
        I::Err: Error + 'static,
    {
        self.0
            .insert(dev_command.name.to_string(), Box::new(dev_command));
        self
    }
}

pub struct DevCommand<I>
where
    I: FromStr + Send + Sync + 'static,
{
    name: &'static str,
    pub(super) system_id: SystemId<In<I>, ()>,
}

impl<I> DevCommand<I>
where
    I: FromStr + Send + Sync + 'static,
{
    pub fn new(
        name: &'static str,
        system: impl IntoSystem<In<I>, (), ()> + 'static,
        world: &mut World,
    ) -> Self {
        Self {
            name,
            system_id: world.register_system(system),
        }
    }
}

pub(super) trait Runnable {
    fn run(&self, commands: &mut Commands, args: &str) -> Option<String>;
    fn prefix(&self) -> &'static str;
}

impl<I> Runnable for DevCommand<I>
where
    I: FromStr + Send + Sync + 'static,
    I::Err: Error + 'static,
{
    fn run(&self, commands: &mut Commands, args: &str) -> Option<String> {
        match args.parse() {
            Ok(args) => {
                commands.run_system_with_input(self.system_id, args);
                None
            }
            Err(e) => {
                warn!("Error running command {}: {}", self.name, e);
                Some(e.to_string())
            }
        }
    }

    fn prefix(&self) -> &'static str {
        self.name
    }
}
