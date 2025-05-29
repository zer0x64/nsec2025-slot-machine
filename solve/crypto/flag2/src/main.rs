use std::collections::VecDeque;
use std::io::stdout;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use anyhow::Result;
use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::{Attribute, Color, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, SetSize};
use crossterm::ExecutableCommand;
use futures::{select, FutureExt, StreamExt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tokio::pin;

struct Terminal {
    cols: u16,
    rows: u16,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let (cols, rows) = size()?;
        enable_raw_mode()?;
        Ok(Terminal { cols, rows })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().expect("Couldn't disable raw mode!");
        stdout()
            .execute(SetSize(self.cols, self.rows))
            .expect("Couldn't set back terminal size!");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Keeps a handle to call drop at the end
    let _terminal = Terminal::new()?;

    // Allow user to adjust the timestamp
    loop {
        stdout()
            .execute(Clear(crossterm::terminal::ClearType::All))?
            .execute(MoveTo(0, 0))?
            .execute(Print(
                <chrono::DateTime<chrono::Local>>::from(
                    chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap(),
                )
                .format("%H:%M:%S")
                .to_string(),
            ))?;

        match crossterm::event::read()? {
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                timestamp += 1;
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                timestamp -= 1;
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                break;
            }
            _ => {}
        }
    }

    let seed = timestamp;
    let mut spins = 0;
    let mut view_offset = 0;
    let mut auto_advance = true;

    // Generate initial values
    let mut values = VecDeque::new();
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..20 {
        values.push_back(rng.random::<u8>());
    }

    let (sender, mut receiver) = tokio::sync::mpsc::channel(10);

    // Thread to send a signal every second for auto-advancing
    thread::spawn(move || {
        // Sync to the next second
        thread::sleep(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp))
                .unwrap(),
        );

        loop {
            let now = Instant::now();

            if sender.blocking_send(()).is_err() {
                println!("Channel closed, thread stopped...");
                break;
            }

            thread::sleep((now + Duration::from_secs(1)).duration_since(Instant::now()));
        }
    });

    let mut event_stream = crossterm::event::EventStream::new();

    loop {
        // Recreate RNG state based on seed, spins, and view_offset
        let mut rng = StdRng::seed_from_u64(seed);

        // First advance RNG according to spins
        for _ in 0..(spins * 3) {
            let _ = rng.random::<u8>();
        }

        // Store values after spins for our sliding window
        let mut all_values = Vec::new();
        for _ in 0..(20 + view_offset) {
            all_values.push(rng.random::<u8>());
        }

        // Get our visible window of values
        let current_values: Vec<u8> = all_values
            .iter()
            .skip(view_offset)
            .take(20)
            .cloned()
            .collect();

        let values_str: Vec<String> = current_values
            .iter()
            .map(|v| {
                match v {
                    0..=127 => "c",
                    128..=191 => "1",
                    192..=217 => "2",
                    218..=239 => "3",
                    240..=248 => "S",
                    249..=252 => "-",
                    253..=254 => "+",
                    255 => "!",
                }
                .to_string()
            })
            .collect();

        let mut matches: Vec<(usize, usize, Color)> = Vec::new();

        for (i, w) in values_str.windows(3).enumerate() {
            if w[0] == w[1] && w[0] == w[2] {
                // This is a match!
                let color = match w[0].as_str() {
                    "c" => Color::Green,
                    "1" | "2" | "3" => Color::Cyan,
                    "-" | "+" | "!" => Color::Red,
                    _ => Color::White,
                };

                matches.push((i, i + 2, color));
            }
        }

        stdout()
            .execute(Clear(crossterm::terminal::ClearType::All))?
            .execute(MoveTo(0, 0))?;

        // Display current status
        stdout().execute(Print(format!(
            "Seed: {}, Spins: {}, Offset: {}, Auto: {}",
            seed,
            spins,
            view_offset,
            if auto_advance { "ON" } else { "OFF" }
        )))?;
        stdout().execute(MoveTo(0, 1))?;

        // Display symbols
        for (i, v) in values_str.into_iter().enumerate() {
            for m in &matches {
                if m.0 == i {
                    stdout()
                        .execute(SetForegroundColor(m.2))?
                        .execute(SetAttribute(Attribute::Bold))?;
                }
            }

            stdout().execute(Print(v))?.execute(Print("  "))?;

            for m in &matches {
                if m.1 == i {
                    stdout()
                        .execute(crossterm::style::SetForegroundColor(Color::White))?
                        .execute(SetAttribute(Attribute::NoBold))?;
                }
            }
        }

        // Display second positions (actual clock seconds) under each symbol
        stdout().execute(MoveTo(0, 2))?;
        for i in 0..20 {
            // Calculate seconds based only on view_offset, not spins
            let position_second = (seed as usize % 60 + view_offset + i) % 60;
            stdout().execute(Print(format!("{:02} ", position_second)))?;
        }

        // Display controls
        stdout().execute(MoveTo(0, 4))?;
        stdout().execute(Print(
            "Controls: Enter=Spin, Left/Right=Move View, S=Toggle Auto, +/-=Adjust Spins, Esc=Exit",
        ))?;

        let mut event = event_stream.next().fuse();
        let chan = receiver.recv().fuse();

        pin!(chan);

        select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    }))) => {
                        // Increment spins (spin the machine)
                        spins += 1;
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }))) => {
                        // Move view window forward
                        view_offset += 1;
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }))) => {
                        // Move view window backward
                        if view_offset > 0 {
                            view_offset -= 1;
                        }
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Char('s') | KeyCode::Char('S'),
                        ..
                    }))) => {
                        // Toggle auto advance
                        auto_advance = !auto_advance;
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Char('+'),
                        ..
                    }))) => {
                        // Increase spins
                        spins += 1;
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Char('-'),
                        ..
                    }))) => {
                        // Decrease spins
                        if spins > 0 {
                            spins -= 1;
                        }
                    }
                    Some(Ok(crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    }))) => {
                        break;
                    }
                    None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            },
            _ = chan => {
                // Auto advance if enabled
                if auto_advance {
                    view_offset += 1;
                }
            }
        };
    }

    Ok(())
}
