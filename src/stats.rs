use sysinfo::System;

pub struct SystemStats {
    pub cpu: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpus = sys.cpus();
        let cpu = if cpus.is_empty() {
            0.0
        } else {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        };

        let ram_used = sys.used_memory();
        let ram_total = sys.total_memory();

        let (disk_used, disk_total) = get_disk_usage();

        SystemStats {
            cpu,
            ram_used,
            ram_total,
            disk_used,
            disk_total,
        }
    }

    pub fn ram_percent(&self) -> f32 {
        (self.ram_used as f32 / self.ram_total as f32) * 100.0
    }

    pub fn disk_percent(&self) -> f32 {
        if self.disk_total == 0 {
            0.0
        } else {
            (self.disk_used as f32 / self.disk_total as f32) * 100.0
        }
    }
}

fn get_disk_usage() -> (u64, u64) {
    use std::process::Command;

    let output = Command::new("df")
        .args(["-B1", "/"])
        .output()
        .expect("Failed to run df");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() >= 2 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 4 {
            let total: u64 = parts[1].parse().unwrap_or(0);
            let available: u64 = parts[3].parse().unwrap_or(0);
            return (total - available, total);
        }
    }

    (0, 0)
}
