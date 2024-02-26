use std::{io, panic, thread};
use std::io::stderr;
use std::sync::mpsc::Sender;

use crossterm::{event, terminal};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;

pub type CrosstermTerminal = Terminal<ratatui::backend::CrosstermBackend<io::Stderr>>;

pub enum TerminalEvent {
    Key(KeyEvent),
}

pub struct Tui {
    terminal: CrosstermTerminal,
    sender: Sender<TerminalEvent>,
}

impl Tui {
    pub fn new(sender: Sender<TerminalEvent>) -> Self {
        let be = ratatui::backend::CrosstermBackend::new(stderr());
        let terminal = Terminal::new(be).expect("could not init terminal");

        Self {
            terminal,
            sender,
        }
    }

    pub fn enter(&self) {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |p| {
            Self::exit();
            hook(p);
        }));

        terminal::enable_raw_mode().expect("could not enable raw mode");
        crossterm::execute!(
            stderr(),
            EnterAlternateScreen,
            EnableMouseCapture,
        ).expect("could not configure crossterm");

        self.poll_terminal_events();
    }

    fn poll_terminal_events(&self) {
        let sender = self.sender.clone(); // todo
        thread::spawn(move || {
            loop {
                match event::read().expect("could not read events") {
                    Event::Key(e) if e.kind == KeyEventKind::Press => {
                        sender.send(TerminalEvent::Key(e))
                            .expect("could not send TerminalEvent")
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn exit() {
        terminal::disable_raw_mode().expect("could not exit raw mode");
        crossterm::execute!(
            stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        ).expect("could not configure crossterm")
    }
}