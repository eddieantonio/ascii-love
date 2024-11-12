use std::f64::consts::PI;
use std::thread;
use std::time;

const SCREEN_WIDTH: usize = 150;
const SCREEN_HEIGHT: usize = 40;
const LUMINANCE: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

fn main() {
    let pause = time::Duration::from_millis(45);
    let mut a_iter = (0.0..2.0 * PI).by(0.05).cycle();
    let mut b_iter = (0.0..2.0 * PI).by(0.04).cycle();

    loop {
        let a = a_iter.next().unwrap();
        let b = b_iter.next().unwrap();
        clear_screen();
        render_frame(a, b);
        thread::sleep(pause);
    }
}

fn render_frame(a: f64, b: f64) {
    let mut output = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[-f64::INFINITY; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for u in (0.0..2.0 * PI).by(0.02) {
        for v in (0.0..PI).by(0.02) {
            // Heart parametric equations
            let x = sin(v) * (15.0 * sin(u) - 4.0 * sin(3.0 * u));
            let y = 8.0 * cos(v);
            let z =
                sin(v) * (15.0 * cos(u) - 5.0 * cos(2.0 * u) - 2.0 * cos(3.0 * u) - cos(4.0 * u));

            // Rotate around Y-axis
            let x1 = x * cos(b) + z * sin(b);
            let y1 = y;
            let z1 = -x * sin(b) + z * cos(b);

            // Rotate around X-axis
            let x_rot = x1;
            let y_rot = y1 * cos(a) - z1 * sin(a);
            let z_rot = y1 * sin(a) + z1 * cos(a);

            // Projection
            let z_offset = 70.0;
            let ooz = 1.0 / (z_rot + z_offset);
            let width = SCREEN_WIDTH as f64;
            let height = SCREEN_HEIGHT as f64;
            let xp = (width / 2.0 + x_rot * ooz * width) as usize;
            let yp = (height / 2.0 - y_rot * ooz * height) as usize;

            // Calculate normals
            let nx = sin(v) * (15.0 * cos(u) - 4.0 * cos(3.0 * u));
            let ny = 8.0 * -sin(v) * sin(v);
            let nz =
                cos(v) * (15.0 * sin(u) - 5.0 * sin(2.0 * u) - 2.0 * sin(3.0 * u) - sin(4.0 * u));

            // Rotate normals around Y-axis
            let nx1 = nx * cos(b) + nz * sin(b);
            let ny1 = ny;
            let nz1 = -nx * sin(b) + nz * cos(b);

            // Rotate normals around X-axis
            let nx_rot = nx1;
            let ny_rot = ny1 * cos(a) - nz1 * sin(a);
            let nz_rot = ny1 * sin(a) + nz1 * cos(a);

            // Normalize normal vector
            let length = (nx_rot.powi(2) + ny_rot.powi(2) + nz_rot.powi(2)).sqrt();
            let nx_rot = nx_rot / length;
            let ny_rot = ny_rot / length;
            let nz_rot = nz_rot / length;

            // Light direction
            let lx = 0.0;
            let ly = 0.0;
            let lz = -1.0;

            // Dot product for luminance
            let luma = nx_rot * lx + ny_rot * ly + nz_rot * lz;
            let luminance_index = ((luma + 1.0) * 5.5) as i32;

            let within_screen = xp < SCREEN_WIDTH && yp < SCREEN_HEIGHT;
            let visible = ooz > zbuffer[yp][xp];
            if within_screen && visible {
                zbuffer[yp][xp] = ooz;
                let n_lumas = LUMINANCE.len() - 1;
                let luminance_index = luminance_index.clamp(0, n_lumas as i32) as usize;
                output[yp][xp] = LUMINANCE[luminance_index];
            }
        }
    }

    clear_screen();
    for line in output {
        let line: String = line.iter().collect();
        println!("{line}");
    }

    // I would use f64:: these, but I don't believer it's possible because they're primitives.
    #[inline(always)]
    fn sin(x: f64) -> f64 {
        x.sin()
    }

    #[inline(always)]
    fn cos(x: f64) -> f64 {
        x.cos()
    }
}

fn clear_screen() {
    // Cursor Position to top-left
    print!("\x1b[H");
}

#[derive(Clone)]
struct FloatRangeIter {
    start: f64,
    end: f64,
    step: f64,
    current: i64,
    size: i64,
}

impl Iterator for FloatRangeIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current < self.size {
            // Linear interpolation
            let value = self.start + (self.current as f64) * self.step;
            assert!(value >= self.start);
            assert!(value < self.end);
            Some(value)
        } else {
            None
        }
    }
}

trait ToFloatRangeIter {
    fn by(self, step: f64) -> FloatRangeIter;
}

impl ToFloatRangeIter for std::ops::Range<f64> {
    fn by(self, step: f64) -> FloatRangeIter {
        let std::ops::Range { start, end } = self;
        let size = (end - start) / step;

        FloatRangeIter {
            start,
            end,
            step,
            current: 0,
            size: size as i64,
        }
    }
}