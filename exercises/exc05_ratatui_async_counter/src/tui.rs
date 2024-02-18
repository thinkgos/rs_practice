use std::ops::{Deref, DerefMut};
use std::{io, panic};

use anyhow::{anyhow, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;
use tokio::task;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<E> {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Event sender channel.
    #[allow(dead_code)]
    pub event_tx: mpsc::UnboundedSender<E>,
    /// Event receiver channel.
    pub event_rx: mpsc::UnboundedReceiver<E>,
    /// Event handler thread.
    pub task: Option<task::JoinHandle<()>>,
    pub frame_rate: f64,
    pub tick_rate: f64,
}

impl<E> Tui<E> {
    /// Constructs a new instance of [`Tui`].
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut tui = Self {
            terminal: Terminal::new(CrosstermBackend::new(std::io::stderr()))?,
            event_tx: tx,
            event_rx: rx,
            task: None,
            frame_rate: 50.,
            tick_rate: 50.,
        };
        tui.enter()?;
        Ok(tui)
    }

    pub fn start_poll_event<F>(&mut self, f: F)
    where
        F: FnOnce(f64, f64, mpsc::UnboundedSender<E>) -> task::JoinHandle<()>,
    {
        let tx = self.event_tx.clone();
        self.task = Some(f(self.tick_rate, self.frame_rate, tx));
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn recv_event(&mut self) -> Result<E> {
        self.event_rx
            .recv()
            .await
            .ok_or(anyhow!("Unable to get event"))
    }
    /// send the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn send_event(&mut self, e: E) -> Result<()> {
        self.event_tx
            .send(e)
            .map_err(|_e| anyhow!("send event failure!"))?;
        Ok(())
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

impl<E> Drop for Tui<E> {
    fn drop(&mut self) {
        if let Err(e) = self.exit() {
            println!("Exit the terminal interface failed: {}", e);
        }
    }
}

impl<E> Deref for Tui<E> {
    type Target = CrosstermTerminal;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl<E> DerefMut for Tui<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
