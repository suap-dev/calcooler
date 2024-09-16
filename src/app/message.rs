use crate::app::operation::Operation;

#[derive(Debug, Clone)]
pub enum Message {
    // row #1
    Percent,
    ClearEntry,
    Clear,
    Back,

    // row #2
    Reciprocal,
    Square,
    SquareRoot,

    Digit(u32),

    // bottom row
    ToggleSign,
    Comma,
    Calculate,

    Operation(Operation),
}
