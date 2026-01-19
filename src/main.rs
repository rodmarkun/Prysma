mod geometry;
mod math;
mod renderer;

use geometry::{cube, triangle};
use math::{Mat4, Vec4};
use renderer::ScreenBuffer;
use std::thread;
use std::time::Duration;

fn main() {
    let mut buffer = ScreenBuffer::new(80, 24);
    let mut angle = 0.0_f32;
    let fov = 40.0;

    loop {
        buffer.clear();

        let rotation = Mat4::rotation_y(angle);
        let translation = Mat4::translation(0.0, 0.0, 5.0);
        let transform = translation * rotation;

        // Transform all vertices once
        let transformed: Vec<_> = triangle::VERTICES
            .iter()
            .map(|v| transform * Vec4::new(v.x, v.y, v.z, 1.0))
            .collect();

        // Draw edges
        for (i, j) in triangle::EDGES.iter() {
            let p0 = buffer.project_to_screen(transformed[*i], fov);
            let p1 = buffer.project_to_screen(transformed[*j], fov);

            if let (Some((x0, y0)), Some((x1, y1))) = (p0, p1) {
                buffer.draw_line(x0, y0, x1, y1, '#');
            }
        }

        // Draw vertices on top
        for v in &transformed {
            buffer.project_point(*v, fov, '@');
        }

        buffer.print();
        thread::sleep(Duration::from_millis(16));
        angle += 0.05;
    }
}
