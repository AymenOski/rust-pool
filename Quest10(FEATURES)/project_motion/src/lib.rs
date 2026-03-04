#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            init_position,
            init_velocity,
            time: 0.0,
        }
    }
}

fn round1(x: f32) -> f32 {
    (x * 10.0).round() / 10.0
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0;

        let g: f32 = 9.8;
        let t: f32 = self.time;

        let vx = self.init_velocity.x;
        let vy = self.init_velocity.y - g * t;

        let x = self.init_position.x + self.init_velocity.x * t;
        let y = self.init_position.y + self.init_velocity.y * t - 0.5 * g * t * t;

        self.actual_velocity = Object {
            x: round1(vx),
            y: round1(vy),
        };
        self.actual_position = Object {
            x: round1(x),
            y: round1(y),
        };

        if self.actual_position.y <= 0.0 {
            return None;
        }

        Some(self.clone())
    }
}
