use crate::clock;
use crate::spotify::SpotifyStatus;
use crate::stats::SystemStats;

pub fn render(stats: &SystemStats, spotify: &SpotifyStatus, width: usize) {
    let clock_time = chrono::Local::now().format("%H:%M").to_string();
    let clock_lines = clock::render_hour(&clock_time);

    let ram_pct = stats.ram_percent();
    let cpu_pct = stats.cpu;
    let disk_pct = stats.disk_percent();

    let (ram_used, ram_total) = stats.ram_gb();
    let (disk_used, disk_total) = stats.disk_gb();

    let min_width: usize = 50;
    let actual_width = width.max(min_width);
    let inner_width = actual_width - 2;

    let border_top = "─".repeat(inner_width);
    let border_bottom = "─".repeat(inner_width);

    println!("\x1b[36m┌{}┐\x1b[0m", border_top);

    print_stat("Ram", ram_pct, &format!("{:.1}/{:.1}GB", ram_used, ram_total), inner_width);
    print_stat("Cpu", cpu_pct, &format!("{:.0}%", cpu_pct), inner_width);
    print_stat("Disk", disk_pct, &format!("{:.1}/{:.1}GB", disk_used, disk_total), inner_width);

    let empty_line = " ".repeat(inner_width);
    println!("\x1b[36m│\x1b[0m{}\x1b[36m│\x1b[0m", empty_line);

    for line in &clock_lines {
        print_centered(line, inner_width);
    }

    println!("\x1b[36m│\x1b[0m{}\x1b[36m│\x1b[0m", empty_line);

    if spotify.is_running {
        let progress = spotify.progress_percent();
        let bar_width: usize = inner_width.saturating_sub(18);
        let filled = (progress / 100.0 * bar_width as f32) as usize;
        let empty = bar_width.saturating_sub(filled);

        let play_icon = if spotify.is_playing { "▶" } else { "⏸" };
        let next_icon = "⏭";

        let bottom_line = format!(
            " {} ─── {}{} ─── {}",
            play_icon,
            "█".repeat(filled),
            "░".repeat(empty),
            next_icon
        );

        print_centered(&bottom_line, inner_width);

        let title_display = if spotify.title.len() > 25 {
            format!("{}...", &spotify.title[..22])
        } else {
            spotify.title.clone()
        };

        let artist_line = format!(" {} - {}", title_display, spotify.artist);
        print_colored(&artist_line, inner_width, "\x1b[2;37m");
    } else {
        let spotify_msg = "♫ Spotify not running";
        print_colored(spotify_msg, inner_width, "\x1b[2;37m");
    }

    println!("\x1b[36m└{}┘\x1b[0m", border_bottom);
}

fn print_centered(text: &str, width: usize) {
    let text_width = strip_ansi_len(text);

    if text_width >= width {
        println!("\x1b[36m│\x1b[0m{}\x1b[36m│\x1b[0m", text);
        return;
    }

    let padding = (width - text_width) / 2;
    let right_pad = width - text_width - padding;

    println!(
        "\x1b[36m│\x1b[0m{}{}{}\x1b[36m│\x1b[0m",
        " ".repeat(padding),
        text,
        " ".repeat(right_pad)
    );
}

fn print_colored(text: &str, width: usize, color: &str) {
    let text_width = strip_ansi_len(text);

    if text_width >= width {
        println!("\x1b[36m│\x1b[0m{}\x1b[36m│\x1b[0m{}", text, color);
        return;
    }

    let padding = (width - text_width) / 2;
    let right_pad = width - text_width - padding;

    println!(
        "\x1b[36m│\x1b[0m{}{}{}{}\x1b[0m{}\x1b[36m│\x1b[0m",
        " ".repeat(padding),
        color,
        text,
        " ".repeat(right_pad),
        ""
    );
}

fn strip_ansi_len(s: &str) -> usize {
    let mut in_escape = false;
    let mut len = 0;

    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }

    len
}

fn print_stat(name: &str, percent: f32, detail: &str, width: usize) {
    let bar_width: usize = 20;
    let filled = (percent / 100.0 * bar_width as f32) as usize;
    let empty = bar_width.saturating_sub(filled);

    let bar = format!(
        "\x1b[32m{}\x1b[0m{}",
        "█".repeat(filled),
        "░".repeat(empty)
    );

    let stat_line = format!(
        " {}: {:>5.1}% [{}] {}",
        name,
        percent,
        bar,
        detail
    );

    let visible_len = strip_ansi_len(&stat_line);
    let padding = if visible_len < width {
        width - visible_len
    } else {
        0
    };

    println!(
        "\x1b[36m│\x1b[0m\x1b[1;37m{}\x1b[0m{}\x1b[36m│\x1b[0m",
        stat_line,
        " ".repeat(padding)
    );
}
