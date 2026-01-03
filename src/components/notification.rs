pub mod notification {
    use lazy_static::lazy_static;
    use ratatui::{
        layout::Rect,
        style::{Color, Style, Stylize},
        widgets::{Block, Borders, Clear, Paragraph, Wrap},
        Frame,
    };
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    // --- ENUMS ---
    #[derive(Clone, Debug, PartialEq)]
    pub enum Level {
        Info,
        Warning,
        Success,
        Error,
    }

    // --- INTERNAL STATE ---
    pub struct NotificationState {
        pub message: String,
        pub level: Level,
        pub expires_at: Instant,
    }

    lazy_static! {
        pub static ref ACTIVE_NOTIFICATION: Mutex<Option<NotificationState>> = Mutex::new(None);
    }

    // --- OPTIONS STRUCT ---
    #[derive(Clone, Debug)]
    pub struct Options {
        pub duration_ms: u64,
        pub message: String,
        pub level: Level,
    }

    // 1. Define Defaults here
    impl Default for Options {
        fn default() -> Self {
            Options {
                duration_ms: 3000,
                message: String::from("Operation Complete"),
                level: Level::Info,
            }
        }
    }

    impl Options {
        // 2. The show method (pushes to global state)
        pub fn show(self) {
            let mut state = ACTIVE_NOTIFICATION.lock().unwrap();
            *state = Some(NotificationState {
                message: self.message,
                level: self.level,
                expires_at: Instant::now() + Duration::from_millis(self.duration_ms),
            });
        }
    }

    // --- RENDERER (Call in Main Loop) ---
    pub fn render(frame: &mut Frame) {
        let mut state_guard = ACTIVE_NOTIFICATION.lock().unwrap();

        if let Some(state) = state_guard.as_ref() {
            // Check Expiration
            if Instant::now() > state.expires_at {
                *state_guard = None;
                return;
            }

            // Draw Logic
            let area = frame.area();
            let width = 30;
            let height = 5;

            let popup_area = Rect {
                x: area.width.saturating_sub(width + 2),
                y: 2,
                width,
                height,
            };

            let color = match state.level {
                Level::Info => Color::Blue,
                Level::Warning => Color::Yellow,
                Level::Error => Color::Red,
                Level::Success => Color::Green,
            };

            let symbol = match state.level {
                Level::Info => "ℹ️",
                Level::Warning => "⚠️",
                Level::Error => "❌",
                Level::Success => "✅",
            };

            let block = Block::new()
                .title(format!(" {:?} ", state.level))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title_style(Style::new().fg(color));

            let text = Paragraph::new(format!("{} {}", symbol, state.message))
                .wrap(Wrap { trim: true })
                .block(block)
                .bold()
                .style(Style::new().fg(color));

            frame.render_widget(Clear, popup_area);
            frame.render_widget(text, popup_area);
        }
    }
}
