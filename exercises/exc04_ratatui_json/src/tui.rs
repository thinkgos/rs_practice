use std::{
    io,
    ops::{Deref, DerefMut},
    panic,
};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new() -> Result<Self> {
        let mut tui = Self {
            terminal: Terminal::new(CrosstermBackend::new(std::io::stderr()))?,
        };
        tui.enter()?;
        Ok(tui)
    }

    /// Enter the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        Self::init_panic_hook();
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
    /// Initializes the terminal interface panic hook.
    /// the terminal properties if unexpected errors occur
    fn init_panic_hook() {
        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));
    }
    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        if terminal::is_raw_mode_enabled().unwrap_or(true) {
            terminal::disable_raw_mode()?;
            crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        }
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        if let Err(e) = self.exit() {
            println!("Exit the terminal interface failed: {}", e);
        }
    }
}

impl Deref for Tui {
    type Target = CrosstermTerminal;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
