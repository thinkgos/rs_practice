use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use anyhow::{anyhow, Result};
use crossterm::cursor;
use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;
use tokio::task;
use tokio_util::sync::CancellationToken;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.

pub struct Tui<W: Write, E> {
    /// Interface to the Terminal.
    terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>,
    /// Event sender channel.
    event_tx: mpsc::UnboundedSender<E>,
    /// Event receiver channel.
    event_rx: mpsc::UnboundedReceiver<E>,
    /// Event handler thread.
    task: task::JoinHandle<()>,
    cancellation_token: CancellationToken,
    frame_rate: f64,
    tick_rate: f64,
    mouse: bool,
    paste: bool,
}

impl<W: Write, E> Tui<W, E> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(writer: W) -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(writer))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let task: task::JoinHandle<()> = tokio::spawn(async {});

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            task,
            cancellation_token,
            frame_rate: 50.,
            tick_rate: 4.,
            mouse: false,
            paste: false,
        })
    }
    pub fn frame_rate(mut self, rate: f64) -> Self {
        self.frame_rate = rate;
        self
    }
    pub fn tick_rate(mut self, rate: f64) -> Self {
        self.tick_rate = rate;
        self
    }
    pub fn mouse(mut self, mouse: bool) -> Self {
        self.mouse = mouse;
        self
    }
    pub fn paste(mut self, paste: bool) -> Self {
        self.paste = paste;
        self
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

    pub fn enter<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(f64, f64, mpsc::UnboundedSender<E>, CancellationToken) -> task::JoinHandle<()>,
    {
        self.enter_terminal()?;
        // make sure only one task is running
        self.cancellation_token.cancel();
        self.cancellation_token = CancellationToken::new();
        let cancellation_token = self.cancellation_token.clone();
        let tx = self.event_tx.clone();
        self.task = f(self.tick_rate, self.frame_rate, tx, cancellation_token);
        Ok(())
    }
    fn exit(&mut self) -> Result<()> {
        self.cancellation_token.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));

            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 100 {
                log::error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        self.exit_terminal()
    }
    /// Enter the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    fn enter_terminal(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            cursor::Hide
        )?;
        if self.mouse {
            crossterm::execute!(self.terminal.backend_mut(), EnableMouseCapture)?;
        }
        if self.paste {
            crossterm::execute!(self.terminal.backend_mut(), EnableBracketedPaste)?;
        }
        Ok(())
    }
    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    fn exit_terminal(&mut self) -> Result<()> {
        if terminal::is_raw_mode_enabled().unwrap_or(true) {
            if self.paste {
                crossterm::execute!(self.terminal.backend_mut(), DisableBracketedPaste)?;
            }
            if self.mouse {
                crossterm::execute!(self.terminal.backend_mut(), DisableMouseCapture)?;
            }
            crossterm::execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                cursor::Show
            )?;
            terminal::disable_raw_mode()?;
        }
        Ok(())
    }
}
impl<W: Write, E> Deref for Tui<W, E> {
    type Target = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}
impl<W: Write, E> DerefMut for Tui<W, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
impl<W: Write, E> Drop for Tui<W, E> {
    fn drop(&mut self) {
        if let Err(e) = self.exit() {
            log::error!("Exit the terminal interface failed: {}", e);
        }
    }
}
