pub mod handler {
    use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::{io::Result, time::Duration};

    pub fn exit() -> Result<bool> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read()?
            {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }
}
