use leafwing_input_manager::prelude::*;

//TODO: Move this to an input-handling file
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
}