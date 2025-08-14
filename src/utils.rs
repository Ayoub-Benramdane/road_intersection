use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub color: Color,
    pub direction: Direction,
    pub turn: Turn,
    pub dar: bool,
    pub flmorb3: bool,
    pub pause: bool,
}

pub struct TrafficLights {
    pub up: TrafficLight,
    pub down: TrafficLight,
    pub right: TrafficLight,
    pub left: TrafficLight,
}

pub struct TrafficLight {
    pub x: i32,
    pub y: i32,
    pub light: Light,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Light {
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

const SCARLET: Color = Color::from_rgba(255, 36, 0, 255);

impl Vehicle {
    pub fn new(x: i32, y: i32, speed: i32, direction: Direction) -> Self {
        let mut a = Vehicle {
            x,
            y,
            speed,
            color: Vehicle::color(),
            direction,
            turn: Turn::Straight,
            dar: false,
            flmorb3: false,
            pause: false,
        };

        a.turn = match a.color {
            SCARLET => Turn::Straight,
            BLUE => Turn::Left,
            YELLOW => Turn::Right,
            _ => Turn::Straight,
        };
        a
    }

    pub fn color() -> Color {
        let colors = vec![SCARLET, BLUE, YELLOW];
        colors[quad_rand::gen_range(0, colors.len())]
    }

    pub fn update(&mut self, traffic_lights: &mut TrafficLights, all_vehicles: &Vec<Vec<Vehicle>>) {
        match self.direction {
            Direction::Up => {
                if self.y <= 450 && self.y >= 448 {
                    self.pause = true;
                    if !self.flmorb3 {
                        update_traffic_lights(traffic_lights, all_vehicles);
                    }
                }
                if traffic_lights.up.light == Light::Green {
                    self.pause = false;
                } 
                
                if self.pause {
                    return;
                }
                self.y -= self.speed;

                if self.y <= 405 && !self.dar {
                    if self.turn == Turn::Right {
                        self.direction = Direction::Right;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.y <= 355 {
                        self.direction = Direction::Left;
                        self.dar = true;
                    }
                }
                if self.dar {
                    self.flmorb3 = false;
                }
            }
            Direction::Down => {
                if self.y >= 320 && self.y <= 323 {
                    self.pause = true;
                    if !self.flmorb3 {
                        update_traffic_lights(traffic_lights, all_vehicles);
                    }
                }
                if traffic_lights.down.light == Light::Green {
                    self.pause = false;
                    self.flmorb3 = true;
                }
                if self.pause {
                    return;
                }
                self.y += self.speed;
                if self.y >= 355 && !self.dar {
                    if self.turn == Turn::Right {
                        self.direction = Direction::Left;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.y >= 405 {
                        self.direction = Direction::Right;
                        self.dar = true;
                    }
                }
            }
            Direction::Left => {
                if self.x <= 450 && self.x >= 448 {
                    self.pause = true;
                    if !self.flmorb3 {
                        update_traffic_lights(traffic_lights, all_vehicles);
                    }
                }
                if traffic_lights.left.light == Light::Green {
                    self.pause = false;
                    self.flmorb3 = true;
                }
                if self.pause {
                    return;
                }
                self.x -= self.speed;
                if self.x <= 405 && !self.dar {
                    if self.turn == Turn::Right {
                        self.direction = Direction::Up;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.x <= 355 {
                        self.direction = Direction::Down;
                        self.dar = true;
                    }
                }
            }
            Direction::Right => {
                if self.x >= 320 && self.x <= 323 {
                    self.pause = true;
                    if !self.flmorb3 {
                        update_traffic_lights(traffic_lights, all_vehicles);
                    }
                }
                if traffic_lights.right.light == Light::Green {
                    self.pause = false;
                    self.flmorb3 = true;
                } 
                if self.pause {
                    return;
                }
                self.x += self.speed;
                if self.x >= 355 && !self.dar {
                    if self.turn == Turn::Right {
                        self.direction = Direction::Down;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.x >= 405 {
                        self.direction = Direction::Up;
                        self.dar = true;
                    }
                }
            }
        }
        check_in_rond_point(self, traffic_lights);
    }

    pub fn draw(&self) {
        draw_rectangle(self.x as f32, self.y as f32, 30.0, 30.0, self.color);
    }
}

pub fn check_in_rond_point(vehicle: &mut Vehicle, traffic_lights: &mut TrafficLights) {
    if vehicle.x >= 320 && vehicle.x <= 480 && vehicle.y >= 320 && vehicle.y <= 480 {
        if vehicle.x >= 350 && vehicle.x <= 420 && vehicle.y >= 350 && vehicle.y <= 420 {
            traffic_lights.up.light = Light::Red;
            traffic_lights.down.light = Light::Red;
            traffic_lights.left.light = Light::Red;
            traffic_lights.right.light = Light::Red;
        }
    } else {
        vehicle.flmorb3 = false;
    }
}

impl Direction {
    pub fn random() -> Self {
        match quad_rand::gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl TrafficLights {
    pub fn draw(&self) {
        match self.up.light {
            Light::Red => draw_circle(self.up.x as f32, self.up.y as f32, 20.0,  RED),
            Light::Green => draw_circle(self.up.x as f32, self.up.y as f32, 20.0, GREEN),
        }
        match self.down.light {
            Light::Red => draw_circle(self.down.x as f32, self.down.y as f32, 20.0, RED),
            Light::Green =>
                draw_circle(self.down.x as f32, self.down.y as f32, 20.0, GREEN),
        }
        match self.left.light {
            Light::Red => draw_circle(self.left.x as f32, self.left.y as f32, 20.0, RED),
            Light::Green =>
                draw_circle(self.left.x as f32, self.left.y as f32, 20.0, GREEN),
        }
        match self.right.light {
            Light::Red => draw_circle(self.right.x as f32, self.right.y as f32, 20.0, RED),
            Light::Green =>
                draw_circle(self.right.x as f32, self.right.y as f32, 20.0, GREEN),
        }
    }
}

pub fn draw_dashed_line_x(x1: u32, x2: u32, y: u32) {
    let mut x: u32 = x1;
    loop {
        draw_line(x as f32, y as f32, (x as f32) + 50.0, y as f32, 3.0, WHITE);
        x += 60;
        if x >= x2 {
            break;
        }
    }
}

pub fn draw_dashed_line_y(x: u32, y1: u32, y2: u32) {
    let mut y: u32 = y1;
    loop {
        draw_line(x as f32, y as f32, x as f32, (y as f32) + 50.0, 3.0, WHITE);
        y += 60;
        if y >= y2 {
            break;
        }
    }
}

pub fn make_lights() -> TrafficLights {
    TrafficLights {
        down: TrafficLight {
            x: 320,
            y: 320,
            light: Light::Red,
        },
        up: TrafficLight {
            x: 480,
            y: 480,
            light: Light::Red,
        },
        right: TrafficLight {
            x: 320,
            y: 480,
            light: Light::Red,
        },
        left: TrafficLight {
            x: 480,
            y: 320,
            light: Light::Red,
        },
    }
}

pub fn update_traffic_lights(traffic_lights: &mut TrafficLights, all_vehicles: &Vec<Vec<Vehicle>>) {
    if !all_vehicles.is_empty() {
        traffic_lights.up.light = Light::Red;
        traffic_lights.down.light = Light::Red;
        traffic_lights.left.light = Light::Red;
        traffic_lights.right.light = Light::Red;
    }

    let mut index = vec![];
    let mut max_len = 0;
    for (ind, vehicles) in all_vehicles.iter().enumerate() {
        if vehicles.len() > max_len {
            index.clear();
            max_len = vehicles.len();
            index.push(ind);
        } else if vehicles.len() == max_len {
            index.push(ind);
        }
    }

    for i in index {
        if all_vehicles[i].is_empty() {
            continue;
        }
        if all_vehicles[i][0].color == YELLOW {
            if all_vehicles[(i + 2) % 4].len() > 0 && all_vehicles[(i + 2) % 4][0].color == YELLOW {
                traffic_lights.up.light = Light::Green;
                traffic_lights.down.light = Light::Green;
                traffic_lights.left.light = Light::Green;
                traffic_lights.right.light = Light::Green;
            } else {
                ch3l_do(i, traffic_lights);
            }
        } else if
            all_vehicles[(i + 1) % 4].len() > 0 &&
            all_vehicles[(i + 1) % 4][0].color == YELLOW
        {
            ch3l_do((i + 1) % 4, traffic_lights);
        } else if
            all_vehicles[(i + 3) % 4].len() > 0 &&
            all_vehicles[(i + 3) % 4][0].color == YELLOW
        {
            ch3l_do((i + 3) % 4, traffic_lights);
        } else {
            if i == 0 || i == 2 {
                traffic_lights.up.light = Light::Green;
                traffic_lights.down.light = Light::Green;
                traffic_lights.left.light = Light::Red;
                traffic_lights.right.light = Light::Red;
            } else {
                traffic_lights.left.light = Light::Green;
                traffic_lights.right.light = Light::Green;
                traffic_lights.up.light = Light::Red;
                traffic_lights.down.light = Light::Red;
            }
        }
    }
}

pub fn ch3l_do(i: usize, traffic_lights: &mut TrafficLights) {
    match i {
        0 => {
            traffic_lights.up.light = Light::Green;
            traffic_lights.left.light = Light::Green;
            traffic_lights.right.light = Light::Green;
        }
        1 => {
            traffic_lights.left.light = Light::Green;
            traffic_lights.down.light = Light::Green;
            traffic_lights.up.light = Light::Green;
        }
        2 => {
            traffic_lights.down.light = Light::Green;
            traffic_lights.right.light = Light::Green;
            traffic_lights.left.light = Light::Green;
        }
        _ => {
            traffic_lights.right.light = Light::Green;
            traffic_lights.up.light = Light::Green;
            traffic_lights.down.light = Light::Green;
        }
    }
}
