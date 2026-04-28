use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal,
};
use rand::RngExt;
use std::io::{self, Write};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    Waiting,    // Red screen - wait for green
    Ready,      // Green screen - click now!
    TooSoon,    // Clicked during red
    Result,     // Show reaction time
    Summary,    // Show all results at end
}

struct App {
    results: Vec<u128>,
    attempt: usize,
    max_attempts: usize,
    phase: Phase,
    green_time: Option<Instant>,
}

impl App {
    fn new(max_attempts: usize) -> Self {
        Self {
            results: Vec::new(),
            attempt: 0,
            max_attempts,
            phase: Phase::Waiting,
            green_time: None,
        }
    }
}

fn center_text(text: &str, width: u16) -> String {
    let padding = (width as usize).saturating_sub(text.len()) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

fn fill_background(stdout: &mut io::Stdout, color: Color, rows: u16, cols: u16) -> io::Result<()> {
    execute!(stdout, SetBackgroundColor(color))?;
    let blank_line = " ".repeat(cols as usize);
    for row in 0..rows {
        execute!(
            stdout,
            cursor::MoveTo(0, row),
            Print(&blank_line),
        )?;
    }
    Ok(())
}

fn draw_waiting(stdout: &mut io::Stdout, attempt: usize, max: usize) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    fill_background(stdout, Color::Rgb { r: 206, g: 38, b: 54 }, rows, cols)?;

    let mid = rows / 2;
    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 206, g: 38, b: 54 }),
        SetForegroundColor(Color::White),
    )?;

    let title = "Reaction Time Test";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_sub(3)), Print(center_text(title, cols)))?;

    let line = format!("Round {}/{}", attempt + 1, max);
    execute!(stdout, cursor::MoveTo(0, mid.saturating_sub(1)), Print(center_text(&line, cols)))?;

    let instruction = "Wait for green...";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_add(1)), Print(center_text(instruction, cols)))?;

    execute!(stdout, ResetColor)?;
    stdout.flush()
}

fn draw_ready(stdout: &mut io::Stdout) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    fill_background(stdout, Color::Rgb { r: 75, g: 219, b: 106 }, rows, cols)?;

    let mid = rows / 2;
    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 75, g: 219, b: 106 }),
        SetForegroundColor(Color::DarkGreen),
    )?;

    let title = "CLICK!";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_sub(1)), Print(center_text(title, cols)))?;

    let sub = "Press any key NOW!";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_add(1)), Print(center_text(sub, cols)))?;

    execute!(stdout, ResetColor)?;
    stdout.flush()
}

fn draw_too_soon(stdout: &mut io::Stdout) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    fill_background(stdout, Color::Rgb { r: 50, g: 80, b: 180 }, rows, cols)?;

    let mid = rows / 2;
    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 50, g: 80, b: 180 }),
        SetForegroundColor(Color::White),
    )?;

    let title = "Too soon!";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_sub(1)), Print(center_text(title, cols)))?;

    let sub = "Press any key to try again...";
    execute!(stdout, cursor::MoveTo(0, mid.saturating_add(1)), Print(center_text(sub, cols)))?;

    execute!(stdout, ResetColor)?;
    stdout.flush()
}

fn draw_result(stdout: &mut io::Stdout, ms: u128, attempt: usize, max: usize) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    fill_background(stdout, Color::Rgb { r: 43, g: 135, b: 209 }, rows, cols)?;

    let mid = rows / 2;
    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 43, g: 135, b: 209 }),
        SetForegroundColor(Color::White),
    )?;

    let time_str = format!("{} ms", ms);
    execute!(stdout, cursor::MoveTo(0, mid.saturating_sub(2)), Print(center_text(&time_str, cols)))?;

    let rating = match ms {
        0..=150 => "Insane!",
        151..=200 => "Fast!",
        201..=250 => "Average",
        251..=350 => "Slow",
        _ => "Are you asleep?",
    };
    execute!(stdout, cursor::MoveTo(0, mid), Print(center_text(rating, cols)))?;

    let next = if attempt + 1 < max {
        format!("Press any key for round {}/{}...", attempt + 2, max)
    } else {
        "Press any key to see results...".to_string()
    };
    execute!(stdout, cursor::MoveTo(0, mid.saturating_add(2)), Print(center_text(&next, cols)))?;

    execute!(stdout, ResetColor)?;
    stdout.flush()
}

fn draw_summary(stdout: &mut io::Stdout, results: &[u128]) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    fill_background(stdout, Color::Rgb { r: 43, g: 47, b: 58 }, rows, cols)?;

    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 43, g: 47, b: 58 }),
        SetForegroundColor(Color::White),
    )?;

    let total_lines = 4 + results.len() + 4; // title + gap + results + gap + stats + gap + quit
    let start_row = (rows as usize).saturating_sub(total_lines) / 2;
    let mut row = start_row as u16;

    let title = "=== Results ===";
    execute!(stdout, cursor::MoveTo(0, row), Print(center_text(title, cols)))?;
    row += 2;

    for (i, &ms) in results.iter().enumerate() {
        let line = format!("Round {}: {} ms", i + 1, ms);
        execute!(stdout, cursor::MoveTo(0, row), Print(center_text(&line, cols)))?;
        row += 1;
    }
    row += 1;

    let avg = results.iter().sum::<u128>() / results.len() as u128;
    let best = *results.iter().min().unwrap();
    let worst = *results.iter().max().unwrap();

    let avg_line = format!("Average: {} ms", avg);
    let best_line = format!("Best:    {} ms", best);
    let worst_line = format!("Worst:   {} ms", worst);

    execute!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 75, g: 219, b: 106 }),
    )?;
    execute!(stdout, cursor::MoveTo(0, row), Print(center_text(&avg_line, cols)))?;
    row += 1;
    execute!(
        stdout,
        SetForegroundColor(Color::White),
    )?;
    execute!(stdout, cursor::MoveTo(0, row), Print(center_text(&best_line, cols)))?;
    row += 1;
    execute!(stdout, cursor::MoveTo(0, row), Print(center_text(&worst_line, cols)))?;
    row += 2;

    let quit = "[q] Quit  [r] Restart";
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        cursor::MoveTo(0, row),
        Print(center_text(quit, cols)),
    )?;

    execute!(stdout, ResetColor)?;
    stdout.flush()
}

fn run() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide, EnableMouseCapture)?;

    let mut app = App::new(5);
    let mut rng = rand::rng();
    let mut delay_ms: u64 = rng.random_range(1500..5000);
    let mut phase_start = Instant::now();

    draw_waiting(&mut stdout, app.attempt, app.max_attempts)?;

    loop {
        // In Waiting phase, check if it's time to go green
        if app.phase == Phase::Waiting {
            let elapsed = phase_start.elapsed().as_millis() as u64;
            if elapsed >= delay_ms {
                app.phase = Phase::Ready;
                app.green_time = Some(Instant::now());
                draw_ready(&mut stdout)?;
            }
        }

        // Poll with short timeout so we can check timing
        if event::poll(Duration::from_millis(10))? {
            let evt = event::read()?;

            // Check for Ctrl+C globally - always quit
            if let Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers, .. }) = evt {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }

            // Determine if this is an "action" input (key press or mouse click)
            let is_quit = matches!(
                evt,
                Event::Key(KeyEvent { code: KeyCode::Char('q'), .. })
                | Event::Key(KeyEvent { code: KeyCode::Esc, .. })
            );
            let is_restart = matches!(
                evt,
                Event::Key(KeyEvent { code: KeyCode::Char('r'), .. })
            );
            let is_action = matches!(
                evt,
                Event::Key(_)
                | Event::Mouse(MouseEvent { kind: MouseEventKind::Down(_), .. })
            );

            if !is_action {
                continue;
            }

            match app.phase {
                Phase::Waiting => {
                    if is_quit {
                        break;
                    }
                    // Too soon!
                    app.phase = Phase::TooSoon;
                    draw_too_soon(&mut stdout)?;
                }
                Phase::Ready => {
                    let reaction = app.green_time.unwrap().elapsed().as_millis();
                    app.results.push(reaction);
                    app.phase = Phase::Result;
                    draw_result(&mut stdout, reaction, app.attempt, app.max_attempts)?;
                }
                Phase::TooSoon => {
                    if is_quit {
                        break;
                    }
                    // Retry same round
                    app.phase = Phase::Waiting;
                    delay_ms = rng.random_range(1500..5000);
                    phase_start = Instant::now();
                    draw_waiting(&mut stdout, app.attempt, app.max_attempts)?;
                }
                Phase::Result => {
                    if is_quit {
                        break;
                    }
                    app.attempt += 1;
                    if app.attempt >= app.max_attempts {
                        app.phase = Phase::Summary;
                        draw_summary(&mut stdout, &app.results)?;
                    } else {
                        app.phase = Phase::Waiting;
                        delay_ms = rng.random_range(1500..5000);
                        phase_start = Instant::now();
                        draw_waiting(&mut stdout, app.attempt, app.max_attempts)?;
                    }
                }
                Phase::Summary => {
                    if is_quit {
                        break;
                    }
                    if is_restart {
                        app = App::new(5);
                        delay_ms = rng.random_range(1500..5000);
                        phase_start = Instant::now();
                        draw_waiting(&mut stdout, app.attempt, app.max_attempts)?;
                    }
                }
            }
        }
    }

    execute!(stdout, DisableMouseCapture, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
