use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize, SetTitle,
    },
    ExecutableCommand, QueueableCommand, Result,
};
use std::io::{self, stdout, Stdout, Write};

#[derive(Debug)]
pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn with_title(title: &str) -> Result<Self> {
        let mut stdout = stdout();
        stdout
            .queue(EnterAlternateScreen)?
            .queue(Hide)?
            .queue(SetTitle(title))?;
        stdout.flush()?;
        terminal::enable_raw_mode()?;
        Ok(Terminal { stdout })
    }

    pub fn quit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        self.stdout
            .queue(Show)?
            .queue(ResetColor)?
            .queue(LeaveAlternateScreen)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn size(&self) -> Result<(u16, u16)> {
        terminal::size()
    }

    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.stdout.execute(SetTitle(title))?;
        Ok(())
    }

    pub fn set_size(&mut self, width: u16, height: u16) -> Result<()> {
        self.stdout.execute(SetSize(width, height))?;
        Ok(())
    }

    pub fn cls(&mut self) -> Result<()> {
        self.stdout.execute(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn print<S: ToString>(&mut self, x: u16, y: u16, output: S) -> Result<()> {
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(Print(output.to_string()))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn set(
        &mut self,
        x: u16,
        y: u16,
        fg: Option<Color>,
        bg: Option<Color>,
        glyph: char,
    ) -> Result<()> {
        let colors = Colors {
            foreground: fg,
            background: bg,
        };
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetColors(colors))?
            .queue(Print(glyph.to_string()))?
            .queue(ResetColor)?;
        self.stdout.flush()?;
        Ok(())
    }

    /// `set` without flushing and resetting color.
    pub fn set_fast(
        &mut self,
        x: u16,
        y: u16,
        fg: Option<Color>,
        bg: Option<Color>,
        glyph: char,
    ) -> Result<()> {
        let colors = Colors {
            foreground: fg,
            background: bg,
        };
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetColors(colors))?
            .queue(Print(glyph.to_string()))?;
        Ok(())
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal::with_title("bevy").unwrap()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.quit().unwrap()
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}
