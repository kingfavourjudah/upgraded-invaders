use crossterm::{cursor, style::Print, ExecutableCommand};
use rand::Rng;
use std::io::{Result, Write};

pub struct PowerUp {
    pub x: usize,
    pub y: usize,
    pub kind: PowerUpType,
}

pub enum PowerUpType {
    ExtraLife,
    RapidFire,
}

impl PowerUp {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let kind = if rng.gen_bool(0.5) {
            PowerUpType::ExtraLife
        } else {
            PowerUpType::RapidFire
        };

        Self {
            x: rng.gen_range(0..40),
            y: 0,
            kind,
        }
    }



    /// Draws all power-ups to the screen.
    pub fn draw_all(stdout: &mut impl Write, power_ups: &[Self]) -> Result<()> {
        for power_up in power_ups {
            stdout.execute(cursor::MoveTo(power_up.x as u16, power_up.y as u16))?;
            match power_up.kind {
                PowerUpType::ExtraLife => stdout.execute(Print("❤️"))?,    // Explicitly a &str
                PowerUpType::RapidFire => stdout.execute(Print("⚡"))?,     // Explicitly a &str
            };            
        }
        Ok(())
    }
}
