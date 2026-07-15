use std::process::Command;

pub struct SpotifyStatus {
    pub is_running: bool,
    pub is_playing: bool,
    pub title: String,
    pub artist: String,
    pub progress_ms: i32,
    pub duration_ms: i32,
}

impl SpotifyStatus {
    pub fn new() -> Self {
        if !is_spotify_running() {
            return SpotifyStatus {
                is_running: false,
                is_playing: false,
                title: String::new(),
                artist: String::new(),
                progress_ms: 0,
                duration_ms: 0,
            };
        }

        let metadata = get_metadata();
        let is_playing = get_playback_status();

        SpotifyStatus {
            is_running: true,
            is_playing,
            title: metadata.0,
            artist: metadata.1,
            progress_ms: metadata.2,
            duration_ms: metadata.3,
        }
    }

    pub fn progress_percent(&self) -> f32 {
        if self.duration_ms == 0 {
            0.0
        } else {
            (self.progress_ms as f32 / self.duration_ms as f32) * 100.0
        }
    }

    pub fn next() {
        let _ = Command::new("dbus-send")
            .args([
                "--print-reply",
                "--dest=org.mpris.MediaPlayer2.spotify",
                "/org/mpris/MediaPlayer2",
                "org.mpris.MediaPlayer2.Player.Next",
            ])
            .output();
    }

    pub fn play_pause() {
        let _ = Command::new("dbus-send")
            .args([
                "--print-reply",
                "--dest=org.mpris.MediaPlayer2.spotify",
                "/org/mpris/MediaPlayer2",
                "org.mpris.MediaPlayer2.Player.PlayPause",
            ])
            .output();
    }
}

fn is_spotify_running() -> bool {
    Command::new("pgrep")
        .arg("spotify")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn get_playback_status() -> bool {
    let output = Command::new("dbus-send")
        .args([
            "--print-reply",
            "--dest=org.mpris.MediaPlayer2.spotify",
            "/org/mpris/MediaPlayer2",
            "org.freedesktop.DBus.Properties.Get",
            "string:org.mpris.MediaPlayer2.Player",
            "string:PlaybackStatus",
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("Playing")
        }
        Err(_) => false,
    }
}

fn get_metadata() -> (String, String, i32, i32) {
    let output = Command::new("dbus-send")
        .args([
            "--print-reply",
            "--dest=org.mpris.MediaPlayer2.spotify",
            "/org/mpris/MediaPlayer2",
            "org.freedesktop.DBus.Properties.Get",
            "string:org.mpris.MediaPlayer2.Player",
            "string:Metadata",
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            parse_metadata(&stdout)
        }
        Err(_) => (String::new(), String::new(), 0, 0),
    }
}

fn parse_metadata(output: &str) -> (String, String, i32, i32) {
    let mut title = String::new();
    let mut artist = String::new();
    let mut duration = 0i32;
    let mut position = 0i32;

    for line in output.lines() {
        let line = line.trim();
        if line.contains("xesam:title") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start != end {
                        title = line[start + 1..end].to_string();
                    }
                }
            }
        } else if line.contains("xesam:artist") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start != end {
                        artist = line[start + 1..end].to_string();
                    }
                }
            }
        } else if line.contains("mpris:length") {
            if let Some(start) = line.find("uint32 ") {
                let num_str = &line[start + 7..];
                if let Some(end) = num_str.find(char::is_whitespace) {
                    duration = num_str[..end].parse().unwrap_or(0);
                }
            }
        } else if line.contains("Position") {
            if let Some(start) = line.find("int32 ") {
                let num_str = &line[start + 6..];
                if let Some(end) = num_str.find(char::is_whitespace) {
                    position = num_str[..end].parse().unwrap_or(0);
                }
            }
        }
    }

    (title, artist, position, duration)
}
