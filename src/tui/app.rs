use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Pointer,
    io::Write,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal, Terminal,
    crossterm::{
        ExecutableCommand,
        event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    },
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Cell, Paragraph, Row, Table, TableState, Widget},
};

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
                let area = f.area();
                f.render_widget(self.state.borrow().database_state.widget(), area);
            })
            .unwrap();

        let elapsed = start.elapsed();
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
        let mut state = Self {
            database,
            sync_rate,
            last_sync: Instant::now(),
            database_state,
        };
        state.sync();

        return state;
    }
    pub fn sync(&mut self) {}
}
