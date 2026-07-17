mod clock;
mod particles;
mod stats;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame, Terminal,
};

use particles::Particles;
use stats::SystemStats;

fn parse_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "red" => Color::Rgb(255, 80, 80),
        "green" => Color::Rgb(80, 255, 80),
        "yellow" => Color::Rgb(255, 255, 80),
        "blue" => Color::Rgb(80, 160, 255),
        "magenta" => Color::Rgb(255, 80, 255),
        "cyan" => Color::Rgb(80, 255, 255),
        "white" => Color::Rgb(255, 255, 255),
        "gray" | "grey" => Color::Rgb(160, 160, 160),
        _ => Color::Rgb(255, 255, 255),
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let mut color_name = "white".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--color" | "-c" => {
                if i + 1 < args.len() {
                    color_name = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!("BlooFetch - Terminal clock with system stats\n");
                println!("Usage: bloofetch [OPTIONS]\n");
                println!("Options:");
                println!("  -c, --color <COLOR>  Set accent color (red, green, yellow, blue, magenta, cyan, white, gray)");
                println!("  -h, --help           Print help");
                println!("\nControls:");
                println!("  q / Esc              Quit");
                println!("  p                    Toggle particles");
                return Ok(());
            }
            _ => i += 1,
        }
    }

    let accent_color = parse_color(&color_name);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut show_particles = true;
    let mut particle_system = Particles::new(120, 40, 50);

    loop {
        let sys_stats = SystemStats::new();
        let size = terminal.size()?;
        let w = size.width as usize;
        let h = size.height as usize;

        if show_particles {
            particle_system.update(w, h);
        }

        terminal.draw(|f| {
            ui(f, &sys_stats, accent_color, show_particles, &particle_system);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('p') => show_particles = !show_particles,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(
    f: &mut Frame,
    stats: &SystemStats,
    color: Color,
    show_particles: bool,
    particle_system: &Particles,
) {
    let size = f.area();

    if show_particles {
        let mut grid = vec![vec![' '; size.width as usize]; size.height as usize];
        for p in &particle_system.items {
            let px = p.x as usize;
            let py = p.y as usize;
            if px < size.width as usize && py < size.height as usize {
                grid[py][px] = p.char;
            }
        }
        let particle_lines: Vec<Line> = grid
            .iter()
            .map(|row| {
                Line::from(Span::styled(
                    row.iter().collect::<String>(),
                    Style::default().fg(color),
                ))
            })
            .collect();
        let particle_block = Paragraph::new(particle_lines);
        f.render_widget(particle_block, size);
    }

    let ram_pct = stats.ram_percent();
    let cpu_pct = stats.cpu;
    let disk_pct = stats.disk_percent();

    let ram_text = format!("Ram: {:.1}%", ram_pct);
    let cpu_text = format!("Cpu: {:.1}%", cpu_pct);
    let disk_text = format!("Disk: {:.1}%", disk_pct);

    let stats_block = Paragraph::new(vec![
        Line::from(Span::styled(&ram_text, Style::default().fg(color))),
        Line::from(Span::styled(&cpu_text, Style::default().fg(color))),
        Line::from(Span::styled(&disk_text, Style::default().fg(color))),
    ]);

    let stats_area = Rect {
        x: 2,
        y: 2,
        width: 16,
        height: 3,
    };

    f.render_widget(stats_block, stats_area);

    let clock_time = chrono::Local::now().format("%H:%M").to_string();
    let clock_lines = clock::render_time(&clock_time);

    let clock_width = clock_lines[0].chars().count() as u16;
    let clock_height = clock_lines.len() as u16;

    let clock_x = (size.width.saturating_sub(clock_width)) / 2;
    let clock_y = (size.height.saturating_sub(clock_height)) / 2;

    let clock_spans: Vec<Line> = clock_lines
        .iter()
        .map(|line| {
            Line::from(Span::styled(
                line.clone(),
                Style::default().fg(color),
            ))
        })
        .collect();

    let clock_block = Paragraph::new(clock_spans).alignment(Alignment::Center);

    let clock_area = Rect {
        x: clock_x,
        y: clock_y,
        width: clock_width,
        height: clock_height,
    };

    f.render_widget(clock_block, clock_area);

    let particle_status = if show_particles { "ON" } else { "OFF" };
    let hint_text = format!("q: quit  p: particles [{}]", particle_status);
    let hint_x = size.width.saturating_sub(hint_text.len() as u16 + 2);
    let hint_y = size.height.saturating_sub(1);

    let hint_block = Paragraph::new(Span::styled(
        &hint_text,
        Style::default().fg(Color::DarkGray),
    ));

    let hint_area = Rect {
        x: hint_x,
        y: hint_y,
        width: hint_text.len() as u16,
        height: 1,
    };

    f.render_widget(hint_block, hint_area);
}
