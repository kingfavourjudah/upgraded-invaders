use crossterm::{cursor, style::Print, ExecutableCommand};
use rand::Rng;
use std::io::Write;

pub struct Enemy {
    pub x: usize,
    pub y: usize,
}

impl Enemy {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..40),
            y: 0,
        }
    }

    pub fn spawn_enemies(count: usize) -> Vec<Self> {
        (0..count).map(|_| Self::new()).collect()
    }

    pub fn update(&mut self) -> bool {
        self.y += 1;
        self.y < 20
    }

    pub fn draw_all(stdout: &mut impl Write, enemies: &[Self]) -> std::io::Result<()> {
        for enemy in enemies {
            stdout.execute(cursor::MoveTo(enemy.x as u16, enemy.y as u16))?;
            stdout.execute(Print("V"))?;
        }
        Ok(())
    }
}