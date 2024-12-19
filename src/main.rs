mod player;
mod enemy;
mod bullet;
mod power_up;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::{io::{stdout, Write}, thread, time::Duration};
use std::io::Result as IoResult;

use player::Player;
use bullet::Bullet;
use enemy::Enemy;
use power_up::{PowerUp, PowerUpType};

fn quit_game() -> bool {
    // Poll for the ESC key press to quit the game
    if event::poll(Duration::from_millis(50)).unwrap_or(false) {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.code == KeyCode::Esc {
                return true;
            }
        }
    }
    false
}

fn main() -> IoResult<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut player = Player::new();
    let mut bullets = Vec::new();
    let mut enemies = Enemy::spawn_enemies(5);
    let mut power_ups: Vec<PowerUp> = Vec::new();

    let mut frame_count = 0;

    loop {
        // Check for quit condition (ESC key)
        if quit_game() {
            break;
        }

        // Handle user input for player movement and shooting
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                player.handle_input(key_event, &mut bullets);
            }
        }

        // Update game state
        for bullet in &mut bullets {
            bullet.update();
        }
        for enemy in &mut enemies {
            enemy.update();
        }

        // Retain bullets and enemies that should remain in the game
        bullets.retain(|bullet| bullet.y < 20); // Retain bullets that are on screen
        enemies.retain(|enemy| enemy.y < 20); // Retain enemies that are on screen

        // Randomly spawn power-ups
        if frame_count % 100 == 0 {
            power_ups.push(PowerUp::new());
        }

        // Check for collisions between player and power-ups
        power_ups.retain(|power_up| {
            if player.x == power_up.x && power_up.y == 19 {
                match power_up.kind {
                    PowerUpType::ExtraLife => player.lives += 1,
                    PowerUpType::RapidFire => player.rapid_fire = true,
                }
                false // Remove power-up after collection
            } else {
                true
            }
        });

        // Render game state
        execute!(stdout, Clear(ClearType::All))?;
        player.draw(&mut stdout)?;
        Bullet::draw_all(&mut stdout, &bullets)?;
        Enemy::draw_all(&mut stdout, &enemies)?;
        PowerUp::draw_all(&mut stdout, &power_ups)?;

        stdout.flush()?;

        if player.lives == 0 {
            break;
        }

        thread::sleep(Duration::from_millis(100));
        frame_count += 1;
    }

    // Cleanup terminal
    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    println!("Game Over! Your Score: {}", player.score);

    Ok(())
}
