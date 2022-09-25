pub mod channel_mgt;
use channel_mgt::ChannelMessage;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::mpsc::{Receiver, Sender};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

pub fn start_tui(rx_to_tui: Receiver<ChannelMessage>, tx_rom_tui: Sender<ChannelMessage>) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend);

    terminal.unwrap().draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    });

    loop {
        thread::sleep(Duration::from_millis(5000));
    }
    // restore terminal
    disable_raw_mode();
    execute!(
        terminal.unwrap().backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    terminal.unwrap().show_cursor();
}
