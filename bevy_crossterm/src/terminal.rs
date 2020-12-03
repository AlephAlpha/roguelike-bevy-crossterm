use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Attributes, Colors, ContentStyle, Print, ResetColor, SetAttributes, SetColors},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, SetSize, SetTitle},
    ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Stdout, Write};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BufferItem {
    glyph: Option<char>,
    colors: Colors,
    attributes: Attributes,
}

impl Default for BufferItem {
    fn default() -> Self {
        BufferItem {
            glyph: None,
            colors: Colors {
                foreground: None,
                background: None,
            },
            attributes: Attributes::default(),
        }
    }
}

#[derive(Debug)]
pub struct Terminal {
    stdout: Stdout,
    size: (u16, u16),
    old_buffer: Vec<BufferItem>,
    new_buffer: Vec<BufferItem>,
}

impl Terminal {
    pub fn with_title(title: &str) -> Result<Self> {
        let mut stdout = stdout();
        let size = terminal::size()?;
        let old_buffer = vec![BufferItem::default(); (size.0 * size.1) as usize];
        let new_buffer = old_buffer.clone();

        stdout
            .queue(EnterAlternateScreen)?
            .queue(Hide)?
            .queue(SetTitle(title))?;
        stdout.flush()?;

        terminal::enable_raw_mode()?;

        Ok(Terminal {
            stdout,
            size,
            old_buffer,
            new_buffer,
        })
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

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.stdout.execute(SetTitle(title))?;
        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.stdout.execute(SetSize(width, height))?;
        let (old_width, old_height) = self.size;
        self.size = (width, height);

        let mut old_buffer = vec![BufferItem::default(); (width * height) as usize];
        let mut new_buffer = old_buffer.clone();

        for x in 0..old_width.min(width) {
            for y in 0..old_height.min(height) {
                let old_index = (y * old_width + x) as usize;
                let index = (y * width + x) as usize;
                old_buffer[index] = self.old_buffer[old_index];
                new_buffer[index] = self.new_buffer[old_index];
            }
        }

        self.old_buffer = old_buffer;
        self.new_buffer = new_buffer;
        Ok(())
    }

    pub fn cls(&mut self) {
        self.new_buffer = vec![BufferItem::default(); (self.size.0 * self.size.1) as usize];
    }

    pub fn print_with_style<S: ToString>(
        &mut self,
        x: u16,
        y: u16,
        content: S,
        style: ContentStyle,
    ) {
        let index = self.pos_to_index(x, y);
        let string = content.to_string();
        for (item, glyph) in self.new_buffer.iter_mut().skip(index).zip(string.chars()) {
            item.glyph = Some(glyph);
            item.colors = Colors {
                foreground: style.foreground_color,
                background: style.background_color,
            };
            item.attributes = style.attributes;
        }
    }

    pub fn print<S: ToString>(&mut self, x: u16, y: u16, content: S) {
        self.print_with_style(x, y, content, ContentStyle::default())
    }

    pub fn put_char_with_style(&mut self, x: u16, y: u16, glyph: char, style: ContentStyle) {
        let index = self.pos_to_index(x, y);
        let item = &mut self.new_buffer[index];
        item.glyph = Some(glyph);
        item.colors = Colors {
            foreground: style.foreground_color,
            background: style.background_color,
        };
        item.attributes = style.attributes;
    }

    pub fn put_char_with_color(&mut self, x: u16, y: u16, glyph: char, color: Colors) {
        let style = ContentStyle {
            foreground_color: color.foreground,
            background_color: color.background,
            attributes: Attributes::default(),
        };
        self.put_char_with_style(x, y, glyph, style)
    }

    pub fn put_char(&mut self, x: u16, y: u16, glyph: char) {
        self.put_char_with_style(x, y, glyph, ContentStyle::default())
    }

    pub fn flush(&mut self) -> Result<()> {
        for i in 0..self.old_buffer.len() {
            if self.old_buffer[i] != self.new_buffer[i] {
                let (x, y) = self.index_to_pos(i);
                let item = &self.new_buffer[i];
                self.stdout
                    .queue(MoveTo(x, y))?
                    .queue(SetColors(item.colors))?
                    .queue(SetAttributes(item.attributes))?
                    .queue(Print(item.glyph.unwrap_or(' ')))?;
            }
        }
        self.stdout.flush()?;
        self.old_buffer.clone_from(&self.new_buffer);
        Ok(())
    }

    fn pos_to_index(&self, x: u16, y: u16) -> usize {
        (y * self.size.0 + x) as usize
    }

    fn index_to_pos(&self, index: usize) -> (u16, u16) {
        let x = index as u16 % self.size.0;
        let y = index as u16 / self.size.0;
        (x, y)
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
