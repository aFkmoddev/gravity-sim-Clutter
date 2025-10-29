use minifb::{Window, WindowOptions, Key, MouseMode};
// use rand::Rng;

mod body;
mod types;

use body::Body;
use types::Vector2;

fn main() {
    let width = 1280;
    let height = 720;

    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new("Gravity Sim (Rust)", width, height, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

  
    let mut bodies: Vec<Body> = Vec::new();
    
    bodies.push(Body::new(Vector2 { x: (width/2) as f64, y: (height/2) as f64 }, 1.0e14_f64, 40.0, true, 0x00FFA500));

    
    let mut green = Body::new(Vector2 { x: (width/2 + 200) as f64, y: (height/2) as f64 }, 1.0e11_f64, 12.0, false, 0x0000FF00);
    green.velocity = Vector2 { x: 0.0, y: 90.0 };
    bodies.push(green);

    
    let mut blue = Body::new(Vector2 { x: (width/2 - 250) as f64, y: (height/2) as f64 }, 1.2e11_f64, 14.0, false, 0x000000FF);
    blue.velocity = Vector2 { x: 0.0, y: -70.0 };
    bodies.push(blue);

   
    let mut red = Body::new(Vector2 { x: (width/2) as f64, y: (height/2 + 300) as f64 }, 0.8e11_f64, 10.0, false, 0x00FF0000);
    red.velocity = Vector2 { x: 80.0, y: 0.0 };
    bodies.push(red);

   
    let mut purple = Body::new(Vector2 { x: (width/2) as f64, y: (height/2 - 180) as f64 }, 1.5e11_f64, 16.0, false, 0x00FF00FF);
    purple.velocity = Vector2 { x: -60.0, y: 0.0 };
    bodies.push(purple);

    let mut running = true;
    let time_scale = 1.0_f64;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // input
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            running = !running;
        }
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            bodies.clear();
        }

        if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
                // Find central yellow body (assume first body)
                let sun = &bodies[0];
                let px = mx as f64;
                let py = my as f64;
                let dx = px - sun.position.x;
                let dy = py - sun.position.y;
                let r = (dx * dx + dy * dy).sqrt();
                let g = types::G;
                let v = (g * sun.mass / r).sqrt();
                // Perpendicular direction (for orbit)
                let perp = Vector2 { x: -dy / r, y: dx / r };
                let velocity = Vector2 { x: perp.x * v, y: perp.y * v };
                let mut body = Body::new(Vector2 { x: px, y: py }, 1.0e11_f64, 6.0, false, 0x00FFFFFF);
                body.velocity = velocity;
                bodies.push(body);
            }
        }

        // simulation
        let dt = 1.0 / 60.0;
        if running {
            let scaled_dt = dt * time_scale;
            let bodies_clone = bodies.clone();
            for i in 0..bodies.len() {
                let mut b = bodies[i].clone();
                b.update(scaled_dt, &bodies_clone);
                bodies[i] = b;
            }
        }

        // render
        for px in buffer.iter_mut() { *px = 0x00000000; }

        for b in &bodies {
            draw_circle(&mut buffer, width, height, b.position.x as i32, b.position.y as i32, b.radius as i32, b.color);
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn draw_circle(buffer: &mut [u32], width: usize, height: usize, cx: i32, cy: i32, r: i32, color: u32) {
    let r2 = r * r;
    for y in -r..=r {
        for x in -r..=r {
            if x*x + y*y <= r2 {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < height {
                    buffer[py as usize * width + px as usize] = color | 0xFF000000;
                }
            }
        }
    }
}
