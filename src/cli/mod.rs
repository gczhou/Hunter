
/* public api */
pub use self::utils::sprintln;
pub use self::term::{
    Style,
    Color,
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
    prompt,
    confirm,
    get_terminal_size,
    isatty,
    clear,
    ProgressBar,
    Editor,
};
pub use self::command::Command;

mod command;
mod types;
mod utils;
mod formatting;
mod term;
