//! Have you ever been disappointed when a terminal library for rust was only written for UNIX systems?
//! Crossterm provides clearing, input handling, styling, cursor movement, and terminal actions for both Windows and UNIX systems.
//!
//! Crossterm aims to be simple and easy to call in code.
//! Through the simplicity of Crossterm, you do not have to worry about the platform you are working with.
//!
//! This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info).
//!
//! This crate consists of five modules that are provided behind [feature flags](https://timonpost.github.io/crossterm/docs/feature_flags.html) so that you can define which features you'd like to have; by default, all features are enabled.
//! - [Crossterm Style](https://crates.io/crates/crossterm_style)
//! - [Crossterm Input](https://crates.io/crates/crossterm_input)
//! - [Crossterm Screen](https://crates.io/crates/crossterm_screen)
//! - [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
//! - [Crossterm Terminal](https://crates.io/crates/crossterm_terminal)

extern crate crossterm_utils;

#[cfg(feature = "cursor")]
extern crate crossterm_cursor;
#[cfg(feature = "input")]
extern crate crossterm_input;
#[cfg(feature = "screen")]
extern crate crossterm_screen;
#[cfg(feature = "style")]
extern crate crossterm_style;
#[cfg(feature = "terminal")]
extern crate crossterm_terminal;

mod crossterm;

#[cfg(feature = "cursor")]
pub use self::crossterm_cursor::{
    cursor, BlinkOff, BlinkOn, Down, Goto, Hide, Left, ResetPos, Right, SavePos, Show,
    TerminalCursor, Up,
};
#[cfg(feature = "input")]
pub use self::crossterm_input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};
#[cfg(feature = "screen")]
pub use self::crossterm_screen::{AlternateScreen, IntoRawMode, RawScreen};
#[cfg(feature = "style")]
pub use self::crossterm_style::{
    color, style, Attribute, Color, Colored, Colorize, ObjectStyle, PrintStyledFont, SetAttr,
    SetBg, SetFg, StyledObject, Styler, TerminalColor,
};
#[cfg(feature = "terminal")]
pub use self::crossterm_terminal::{
    terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal,
};

pub use self::crossterm::Crossterm;
pub use self::crossterm_utils::{
    execute, queue, Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result,
};
