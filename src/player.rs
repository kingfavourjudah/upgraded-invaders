use crate::bullet::Bullet;
use crossterm::{cursor, style::Print, ExecutableCommand};
use std::io::Write;

pub struct Player {
    pub x: usize,
    pub lives: u8,
    pub score: u32,
    pub rapid_fire: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 20,
            lives: 3,
            score: 0,
            rapid_fire: false,
        }
    }

    pub fn handle_input(&mut self, key_event: crossterm::event::KeyEvent, bullets: &mut Vec<Bullet>) {
        match key_event.code {
            crossterm::event::KeyCode::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.x < 40 {
                    self.x += 1;
                }
            }
            crossterm::event::KeyCode::Char(' ') => {
                bullets.push(Bullet::new(self.x, 18));
                if self.rapid_fire {
                    bullets.push(Bullet::new(self.x + 1, 18));
                    bullets.push(Bullet::new(self.x.wrapping_sub(1), 18));
                }
            }
            _ => {}
        }
    }

   
    pub fn draw(&self, stdout: &mut impl Write) -> std::io::Result<()> {       stdout.execute(cursor::MoveTo(self.x as u16, 19))?;
        stdout.execute(Print("A"))?;
        Ok(())
    }
}
