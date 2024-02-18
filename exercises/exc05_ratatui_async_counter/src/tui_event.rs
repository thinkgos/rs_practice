use std::time::Duration;

use crossterm::event::{EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub fn poll_event(
    tick_rate: f64,
    frame_rate: f64,
    tx: mpsc::UnboundedSender<Event>,
    cancellation_token: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut event_reader = EventStream::new();
        let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / tick_rate));
        let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / frame_rate));
        loop {
            let tick_delay = tick_interval.tick();
            let render_delay = render_interval.tick();
            let crossterm_event = event_reader.next().fuse();
            tokio::select! {
              maybe_event = crossterm_event => {
                match maybe_event {
                  Some(Ok(evt)) => {
                    match evt {
                      crossterm::event::Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            tx.send(Event::Key(key)).unwrap();
                        }
                      },
                      _ => {
                        todo!();
                      },
                    }
                  }
                  Some(Err(_)) => {
                    tx.send(Event::Error).unwrap();
                  }
                  None => {},
                }
              },
              _ = tick_delay => {
                tx.send(Event::Tick).unwrap();
              },
              _ = render_delay => {
                tx.send(Event::Render).unwrap();
              },
              _ = cancellation_token.cancelled() => {
                return
              },
            }
        }
    })
}
