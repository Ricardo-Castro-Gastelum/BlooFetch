mod clock;
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
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use stats::SystemStats;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let sys_stats = SystemStats::new();

        terminal.draw(|f| {
            ui(f, &sys_stats);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
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

fn ui(f: &mut Frame, stats: &SystemStats) {
    let size = f.area();

    f.render_widget(Block::default().style(Style::default().bg(Color::Black)), size);

    let clock_time = chrono::Local::now().format("%H:%M").to_string();
    let clock_lines = clock::render_hour(&clock_time);

    let ram_pct = stats.ram_percent();
    let cpu_pct = stats.cpu;
    let disk_pct = stats.disk_percent();

    let stats_x = 4;
    let stats_y = 4;

    let ram_text = format!("Ram: {:.1}%", ram_pct);
    let cpu_text = format!("Cpu: {:.1}%", cpu_pct);
    let disk_text = format!("Disk: {:.1}%", disk_pct);

    let stats_block = Paragraph::new(vec![
        Line::from(Span::styled(&ram_text, Style::default().fg(Color::Cyan))),
        Line::from(Span::styled(&cpu_text, Style::default().fg(Color::Green))),
        Line::from(Span::styled(&disk_text, Style::default().fg(Color::Yellow))),
    ]);

    let stats_area = Rect {
        x: stats_x,
        y: stats_y,
        width: 15,
        height: 3,
    };

    f.render_widget(stats_block, stats_area);

    let clock_width = clock_lines[0].len() as u16;
    let clock_height = clock_lines.len() as u16;

    let clock_x = (size.width.saturating_sub(clock_width)) / 2;
    let clock_y = (size.height.saturating_sub(clock_height)) / 2;

    let clock_spans: Vec<Line> = clock_lines
        .iter()
        .map(|line| {
            Line::from(Span::styled(
                line.clone(),
                Style::default().fg(Color::White),
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

    let hint_text = "Press q to quit";
    let hint_x = size.width.saturating_sub(hint_text.len() as u16 + 4);
    let hint_y = size.height.saturating_sub(2);

    let hint_block = Paragraph::new(Span::styled(
        hint_text,
        Style::default().fg(Color::DarkGray),
    ));

    let hint_area = Rect {
        x: hint_x,
        y: hint_y,
        width: hint_text.len() as u16 + 2,
        height: 1,
    };

    f.render_widget(hint_block, hint_area);
}
