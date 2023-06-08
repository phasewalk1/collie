use serde::{Deserialize, Serialize};
/// Collie translates input into commands for various backends and executes them.
pub(crate) trait CollieInput
where
    Self: Sized + Serialize + Deserialize<'static>,
{
    /// A valid `CollieInput` must be able to execute itself if it has a mutable reference to a
    /// `CollieCommand` that can bundle its execution.
    fn execute<COMMAND>(&self, command: &mut COMMAND) -> Result<(), COMMAND::Error>
    where
        COMMAND: CollieCommand<Self>,
    {
        command.put_args(self.map_onto());
        command.execute()
    }
    /// A valid `CollieInput` must be able to map itself onto pure arguments.
    fn map_onto(&self) -> Vec<String>;
}

pub(crate) trait CollieCommand<INPUT>
where
    INPUT: CollieInput,
{
    type Error;
    fn execute(&self) -> Result<(), Self::Error>;
    fn put_args(&mut self, args: Vec<String>);
}

/// Reflect CollieInput directly into an executable command.
pub(crate) trait Reflect<INPUT>
where
    INPUT: CollieInput,
{
    type Error;
    fn translate<COMMAND>(&self, input: INPUT) -> Result<COMMAND, Self::Error>
    where
        COMMAND: CollieCommand<INPUT>;
}
