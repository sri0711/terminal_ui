pub mod handler {
    use std::io::Result;

    use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

    pub fn exit() -> Result<bool> {
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
        Ok(false)
    }
}
