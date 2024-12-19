use crossterm::{cursor, style::Print, ExecutableCommand};
use std::io::Write;

pub struct Bullet {
    pub x: usize,
    pub y: usize,
}

impl Bullet {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self) -> bool {
        if self.y > 0 {
            self.y -= 1;
            true
        } else {
            false
        }
    }

    pub fn draw_all(stdout: &mut impl Write, bullets: &[Self]) -> std::io::Result<()> {
        for bullet in bullets {
            stdout.execute(cursor::MoveTo(bullet.x as u16, bullet.y as u16))?;
            stdout.execute(Print("|"))?;
        }
        Ok(())
    }
}