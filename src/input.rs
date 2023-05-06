use leafwing_input_manager::prelude::*;
use strum_macros::{Display, EnumIter};

//TODO: Move this to an input-handling file
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter, Display)]
pub enum Action {
    Exit,
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
    Sprint,
}
