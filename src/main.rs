mod utils;
use crate::utils::*;
use macroquad::prelude::*;

#[macroquad::main("road_insertion")]
async fn main() {
    let mut vehicles_up: Vec<Vehicle> = vec![];
    let mut vehicles_down: Vec<Vehicle> = vec![];
    let mut vehicles_left: Vec<Vehicle> = vec![];
    let mut vehicles_right: Vec<Vehicle> = vec![];
    let mut traffic_lights = make_lights();
    let mut flmorb3 = false;
    request_new_screen_size(800.0, 800.0);
    loop {
        println!("{flmorb3}");

        vehicles_up.retain(|v| v.x >= -32 && v.x <= 800 && v.y >= -32 && v.y <= 800);
        vehicles_down.retain(|v| v.x >= -32 && v.x <= 800 && v.y >= -32 && v.y <= 800);
        vehicles_right.retain(|v| v.x >= -32 && v.x <= 800 && v.y >= -32 && v.y <= 800);
        vehicles_left.retain(|v| v.x >= -32 && v.x <= 800 && v.y >= -32 && v.y <= 800);

        clear_background(BLACK);
        draw_rectangle(350.0, 0.0, 100.0, 800.0, GRAY);
        draw_rectangle(0.0, 350.0, 800.0, 100.0, GRAY);
        draw_dashed_line_x(0, 350, 400);
        draw_dashed_line_x(450, 800, 400);
        draw_dashed_line_y(400, 0, 350);
        draw_dashed_line_y(400, 450, 800);
        draw_circle(400.0, 400.0, 5.0, WHITE);

        traffic_lights.draw();

        if is_key_down(KeyCode::Up) {
            if vehicles_up.len() == 0 || vehicles_up[vehicles_up.len() - 1].y <= 705 {
                vehicles_up.push(Vehicle::new(425, 800, 1, Direction::Up));
            }
        } else if is_key_down(KeyCode::Down) {
            if vehicles_down.len() == 0 || vehicles_down[vehicles_down.len() - 1].y >= 95 {
                vehicles_down.push(Vehicle::new(375, 0, 1, Direction::Down));
            }
        } else if is_key_down(KeyCode::Left) {
            if vehicles_left.len() == 0 || vehicles_left[vehicles_left.len() - 1].x <= 705 {
                vehicles_left.push(Vehicle::new(800, 375, 1, Direction::Left));
            }
        } else if is_key_down(KeyCode::Right) {
            if vehicles_right.len() == 0 || vehicles_right[vehicles_right.len() - 1].x >= 95 {
                vehicles_right.push(Vehicle::new(0, 425, 1, Direction::Right));
            }
        } else if is_key_pressed(KeyCode::R) {
            let direction = Direction::random();
            match direction {
                Direction::Up => {
                    if vehicles_up.len() == 0 || vehicles_up[vehicles_up.len() - 1].y <= 705 {
                vehicles_up.push(Vehicle::new(425, 800, 1, Direction::Up));
            }
                }
                Direction::Down => {
                    if vehicles_down.len() == 0 || vehicles_down[vehicles_down.len() - 1].y >= 95 {
                vehicles_down.push(Vehicle::new(375, 0, 1, Direction::Down));
            }
                }
                Direction::Left => {
                    if vehicles_left.len() == 0 || vehicles_left[vehicles_left.len() - 1].x <= 705 {
                vehicles_left.push(Vehicle::new(800, 375, 1, Direction::Left));
            }
                }
                Direction::Right => {
                    if vehicles_right.len() == 0 || vehicles_right[vehicles_right.len() - 1].x >= 95 {
                vehicles_right.push(Vehicle::new(0, 425, 1, Direction::Right));
            }
                }
            }
        }

        let mut all_vehicles: Vec<Vec<Vehicle>> = vec![
            vehicles_up.clone(),
            vehicles_left.clone(),
            vehicles_down.clone(),
            vehicles_right.clone()
        ];

        // UP (already handled earlier)
        if !vehicles_up.is_empty() {
            vehicles_up[0].update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            vehicles_up[0].draw();

            for i in 1..vehicles_up.len() {
                let (head, tail) = vehicles_up.split_at_mut(i);
                let prev = &head[i - 1];
                let curr = &mut tail[0];

                curr.draw();

                if curr.direction == prev.direction {
                    match curr.direction {
                        Direction::Up => {
                            if curr.y - prev.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Down => {
                            if prev.y - curr.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Left => {
                            if curr.x - prev.x <= 50 {
                                continue;
                            }
                        }
                        Direction::Right => {
                            if prev.x - curr.x <= 50 {
                                continue;
                            }
                        }
                    }
                }

                curr.update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            }
        }

        // DOWN
        if !vehicles_down.is_empty() {
            vehicles_down[0].update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            vehicles_down[0].draw();

            for i in 1..vehicles_down.len() {
                let (head, tail) = vehicles_down.split_at_mut(i);
                let prev = &head[i - 1];
                let curr = &mut tail[0];

                curr.draw();
                if curr.direction == prev.direction {
                    match curr.direction {
                        Direction::Up => {
                            if curr.y - prev.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Down => {
                            if prev.y - curr.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Left => {
                            if curr.x - prev.x <= 50 {
                                continue;
                            }
                        }
                        Direction::Right => {
                            if prev.x - curr.x <= 50 {
                                continue;
                            }
                        }
                    }
                }

                curr.update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            }
        }

        // LEFT
        if !vehicles_left.is_empty() {
            vehicles_left[0].update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            vehicles_left[0].draw();

            for i in 1..vehicles_left.len() {
                let (head, tail) = vehicles_left.split_at_mut(i);
                let prev = &head[i - 1];
                let curr = &mut tail[0];

                curr.draw();
                if curr.direction == prev.direction {
                    match curr.direction {
                        Direction::Up => {
                            if curr.y - prev.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Down => {
                            if prev.y - curr.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Left => {
                            if curr.x - prev.x <= 50 {
                                continue;
                            }
                        }
                        Direction::Right => {
                            if prev.x - curr.x <= 50 {
                                continue;
                            }
                        }
                    }
                }

                curr.update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            }
        }

        // RIGHTR{
        if !vehicles_right.is_empty() {
            vehicles_right[0].update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            vehicles_right[0].draw();

            for i in 1..vehicles_right.len() {
                let (head, tail) = vehicles_right.split_at_mut(i);
                let prev = &head[i - 1];
                let curr = &mut tail[0];

                curr.draw();
                if curr.direction == prev.direction {
                    match curr.direction {
                        Direction::Up => {
                            if curr.y - prev.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Down => {
                            if prev.y - curr.y <= 50 {
                                continue;
                            }
                        }
                        Direction::Left => {
                            if curr.x - prev.x <= 50 {
                                continue;
                            }
                        }
                        Direction::Right => {
                            if prev.x - curr.x <= 50 {
                                continue;
                            }
                        }
                    }
                }

                curr.update(&mut traffic_lights, &mut all_vehicles, &mut flmorb3);
            }
        }

        vehicles_up.retain(|c| c.y > -40);
        vehicles_down.retain(|c| c.y < 800);
        vehicles_left.retain(|c| c.x > -40);
        vehicles_right.retain(|c| c.x < 800);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
