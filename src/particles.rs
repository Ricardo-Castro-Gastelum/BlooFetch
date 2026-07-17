pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub char: char,
}

pub struct Particles {
    pub items: Vec<Particle>,
    seed: u64,
}

impl Particles {
    pub fn new(width: usize, height: usize, count: usize) -> Self {
        let mut p = Particles {
            items: Vec::new(),
            seed: 12345,
        };

        for i in 0..count {
            let x = p.rand(width);
            let y = p.rand(height);
            let speed = 0.2 + p.rand(10) as f64 / 10.0;
            let ch = match i % 5 {
                0 => '.',
                1 => '*',
                2 => '+',
                3 => '~',
                _ => 'o',
            };
            p.items.push(Particle { x, y, speed, char: ch });
        }

        p
    }

    fn rand(&mut self, max: usize) -> f64 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.seed >> 16) % 32768) as f64 / 32768.0 * max as f64
    }

    pub fn update(&mut self, width: usize, height: usize) {
        for i in 0..self.items.len() {
            self.items[i].y -= self.items[i].speed;
            if self.items[i].y < 0.0 {
                self.items[i].y = height as f64;
                self.items[i].x = self.rand(width);
                self.items[i].speed = 0.2 + self.rand(10) as f64 / 10.0;
            }
        }
    }
}
