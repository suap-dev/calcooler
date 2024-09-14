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
    Divide,

    Digit(u8),

    Multiply,
    Subtract,
    Add,

    // bottom row
    ToggleSign,
    Comma,
    Calculate,
}
