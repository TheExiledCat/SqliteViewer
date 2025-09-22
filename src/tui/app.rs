use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Pointer,
    io::Write,
    rc::Rc,
    thread,
    time::{Duration, Instant, SystemTime},
};

use humantime::format_duration;
use ratatui::{
    DefaultTerminal, Terminal,
    crossterm::{
        ExecutableCommand,
        event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read},
    },
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Cell, Paragraph, Row, Table, TableState, Widget},
};
use time_humanize::HumanTime;

use crate::data::sqlite_database::{SqliteDatabase, SqliteDatabaseState};

pub struct App {
    terminal: DefaultTerminal,

    // TODO REPLACE WITH INPUT HANDLER THREADING
    handler: bool,
    fps: u16,
    state: Rc<RefCell<AppState>>,
}
impl App {
    pub fn new(
        terminal: DefaultTerminal,
        fps: u16,
        sync_rate: u16,
        database: SqliteDatabase,
    ) -> Self {
        return Self {
            terminal,
            handler: false,
            fps,
            state: Rc::new(RefCell::new(AppState::new(database, sync_rate))),
        };
    }
    pub fn draw(&mut self) -> bool {
        let mut render_next = true;
        let start = Instant::now();
        let frame_time = Duration::from_secs_f64(1.0 / self.fps as f64);
        self.terminal
            .draw(|f| {
                let event = poll(Duration::from_secs(0));
                if let Ok(true) = event {
                    let event = read();
                    if let Ok(Event::Key(key_event)) = &event {
                        if let KeyCode::Char('q') = key_event.code
                            && key_event.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            render_next = false;
                        }
                    }
                    if let Ok(Event::Key(event)) = &event {
                        let database_state = &mut self.state.borrow_mut().database_state;
                        database_state.read_keys(event);
                    }
                }
                let area = f.area();
                let footer_layout =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
                f.render_widget(
                    self.state.borrow().database_state.widget(),
                    footer_layout[0],
                );
                let database_path = &self.state.borrow().database.database_path;
                let last_sync =
                    Duration::from_secs((Instant::now() - self.state.borrow().last_sync).as_secs());
                let footer = Line::raw(format!(
                    "Last sync: {} ago | Database name: {} | Database path: {:#?}",
                    format_duration(last_sync),
                    database_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .split_once('.')
                        .unwrap()
                        .0,
                    database_path
                ))
                .centered();
                f.render_widget(footer, footer_layout[1]);
            })
            .unwrap();

        let (last_sync, sync_rate, database) = {
            let state = self.state.borrow();
            (state.last_sync, state.sync_rate, state.database.clone())
        };
        let elapsed = start.elapsed();
        if Instant::now() - last_sync >= Duration::from_secs(sync_rate as u64) {
            let mut state = self.state.borrow_mut();
            state.database_state.sync(&database);
            state.last_sync = Instant::now();
        }
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }

        return render_next;
    }
}

pub struct AppState {
    pub database: SqliteDatabase,
    pub sync_rate: u16,
    pub last_sync: Instant,
    pub database_state: SqliteDatabaseState,
}

impl AppState {
    pub fn new(database: SqliteDatabase, sync_rate: u16) -> Self {
        let database_state = SqliteDatabaseState::new(&database);
        let state = Self {
            database,
            sync_rate,
            last_sync: Instant::now(),
            database_state,
        };

        return state;
    }
}
