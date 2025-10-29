use crate::types::Vector2;

#[derive(Clone)]
pub struct Body {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub mass: f64,
    pub radius: f64,
    pub is_static: bool,
    pub color: u32,
    pub speed: f64,
    pub net_force: Vector2,
}

impl Body {
    pub fn new(position: Vector2, mass: f64, radius: f64, is_static: bool, color: u32) -> Self {
        Self {
            position,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass,
            radius,
            is_static,
            color,
            speed: 0.0,
            net_force: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn calculate_shell_gravity_force(&self, other: &Body) -> Vector2 {
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let r = (dx * dx + dy * dy).sqrt();
        let m1 = self.mass;
        let m2 = other.mass;
        let r_shell = other.radius;
        let g = crate::types::G;
        let mut force_magnitude;
        if r >= r_shell {
            force_magnitude = g * m1 * m2 / (r * r);
        } else {
            let effective_mass = m2 * (r / r_shell).powi(3);
            force_magnitude = g * m1 * effective_mass / (r * r);
        }
       
        let min_dist = self.radius + other.radius;
        if r < min_dist {
           
            let repulsion = 1e15 * (min_dist - r).max(0.0);
            force_magnitude -= repulsion;
        }
        let nx = dx / r;
        let ny = dy / r;
        Vector2 { x: force_magnitude * nx, y: force_magnitude * ny }
    }

    pub fn calculate_all_forces(&mut self, bodies: &Vec<Body>) {
        
        // F = G * m1 * m2 / r^2 (plus repulsion if close)
        self.net_force = Vector2 { x: 0.0, y: 0.0 };
        for other in bodies.iter() {
            if (self.position.x == other.position.x) && (self.position.y == other.position.y) {
                continue;
            }
            let f = self.calculate_shell_gravity_force(other);
            self.net_force.x += f.x;
            self.net_force.y += f.y;
        }
        
    }

    pub fn update(&mut self, dt: f64, bodies: &Vec<Body>) {
        if self.is_static { return; }
    
    self.calculate_all_forces(bodies);
    // F = ma -> a = F/m
    
    self.acceleration.x = self.net_force.x / self.mass;
    self.acceleration.y = self.net_force.y / self.mass;
    self.velocity.x += self.acceleration.x * dt;
    self.velocity.y += self.acceleration.y * dt;
    self.position.x += self.velocity.x * dt;
    self.position.y += self.velocity.y * dt;

        let min_x = self.radius;
        let max_x = 1280.0 - self.radius;
        let min_y = self.radius;
        let max_y = 720.0 - self.radius;
        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x = -self.velocity.x.abs();
        }
        if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x = -self.velocity.x.abs();
        }
        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y = -self.velocity.y.abs();
        }
        if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y = -self.velocity.y.abs();
        }

        self.speed = (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt();
        self.net_force = Vector2 { x: 0.0, y: 0.0 };
    }
}
