use std::cmp::max;

pub struct ScreenBuffer {
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![' '; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(' ');
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = c;
        }
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[H");
        for y in 0..self.height {
            let line: String = self.data[y * self.width..(y + 1) * self.width]
                .iter()
                .collect();
            println!("{}", line);
        }
    }

    pub fn project_to_screen(&self, point: crate::math::Vec4, fov: f32) -> Option<(isize, isize)> {
        if point.z > 0.1 {
            let proj_x = point.x * fov / point.z;
            let proj_y = point.y * fov / point.z;

            let screen_x = ((proj_x * 2.0) + (self.width as f32 / 2.0)) as isize;
            let screen_y = ((-proj_y) + (self.height as f32 / 2.0)) as isize;

            Some((screen_x, screen_y))
        } else {
            None
        }
    }

    pub fn project_point(&mut self, point: crate::math::Vec4, fov: f32, c: char) {
        if let Some((x, y)) = self.project_to_screen(point, fov) {
            if x >= 0 && y >= 0 {
                self.set(x as usize, y as usize, c);
            }
        }
    }

    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, c: char) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let steps = max(dx.abs(), dy.abs());

        if steps == 0 {
            self.set(x0 as usize, y0 as usize, c);
            return;
        }

        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;

        let mut x = x0 as f32;
        let mut y = y0 as f32;

        for i in 0..steps {
            self.set(x.round() as usize, y.round() as usize, c);
            x += x_inc;
            y += y_inc;
        }
    }
}
