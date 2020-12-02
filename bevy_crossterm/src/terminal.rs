use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Print, ResetColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
    Command, ExecutableCommand, QueueableCommand, Result,
};
use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};

#[derive(Debug)]
pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn with_title(title: &str) -> Result<Self> {
        let mut stdout = stdout();
        stdout
            .execute(EnterAlternateScreen)?
            .execute(Hide)?
            .execute(SetTitle(title))?;
        terminal::enable_raw_mode()?;
        Ok(Terminal { stdout })
    }

    pub fn quit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        self.stdout
            .execute(Show)?
            .execute(ResetColor)?
            .execute(LeaveAlternateScreen)?;
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

impl<T: Display> ExecutableCommand<T> for Terminal {
    fn execute(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self> {
        self.stdout.execute(command)?;
        Ok(self)
    }
}

impl<T: Display> QueueableCommand<T> for Terminal {
    fn queue(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self> {
        self.stdout.queue(command)?;
        Ok(self)
    }
}
