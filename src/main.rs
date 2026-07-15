mod clock;
mod renderer;
mod spotify;
mod stats;

use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "bloofetch",
    about = "A beautiful terminal decoration with clock, system stats, and Spotify",
    version
)]
struct Cli {
    #[arg(short, long, default_value = "50", help = "Width of the display")]
    width: usize,

    #[arg(short, long, help = "Refresh interval in seconds")]
    interval: Option<u64>,
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn main() {
    let cli = Cli::parse();

    let refresh_rate = cli.interval.unwrap_or(1);

    loop {
        clear_screen();

        let sys_stats = stats::SystemStats::new();
        let spotify_status = spotify::SpotifyStatus::new();

        renderer::render(&sys_stats, &spotify_status, cli.width);

        if cli.interval.is_none() {
            break;
        }

        thread::sleep(Duration::from_secs(refresh_rate));
    }
}
