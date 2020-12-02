use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const BOARD_WIDTH: usize = 16;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

const BOARD_INSIDE: usize = BOARD_WIDTH - 3;

static mut BOARD: [char; BOARD_SIZE] = ['X'; BOARD_SIZE];

const DECELERATION: f32 = -0.1f32;

fn clear_board() {
    for y in 1..(BOARD_WIDTH - 1) {
        for x in 1..(BOARD_WIDTH - 1) {
            unsafe {
                BOARD[x + y * BOARD_WIDTH] = '.';
            }
        }
    }
}

fn print_board() {
    for y in 0..(BOARD_WIDTH) {
        for x in 0..(BOARD_WIDTH) {
            unsafe {
                print!("{} ", BOARD[x + y * BOARD_WIDTH]);
            }
        }
        println!();
    }
}


fn solve_quadratic(a: f32, b: f32, c: f32) -> f32 {
    let determinant: f32 = b * b - 4f32 * a * c;
    // Since we only care about collisions that happen between t=[0, 1)
    // it is ok to return -1 here
    if determinant < 0f32 {
        -1f32
    } else {
        (-b - determinant.sqrt()) / (2f32 * a)
    }
}

fn find_collision_times(start1: Vector, v1: Vector, start2: Vector, v2: Vector, radius: f32) -> f32 {
    let a: f32 = (v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2);

    let b: f32 = -2f32 * (v1.x - v2.x) * (start2.x - start1.x)
        - 2f32 * (v1.y - v2.y) * (start2.y - start1.y);

    let c: f32 = (start1.x - start2.x).powi(2) +
        (start1.y - start2.y).powi(2) - radius.powi(2);

    if a.abs() < f32::EPSILON {
        return -c / b;
    }
    if a.abs() < f32::EPSILON && b.abs() < f32::EPSILON {
        return -1f32;
    }
    solve_quadratic(a, b, c)
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
}


#[derive(Debug, Copy, Clone)]
struct MovingObject {
    velocity: Vector,
    location: Vector,
    ratios: Vector,
    symbol: char,
}

impl MovingObject {
    fn new(starting_velocity: Vector, symbol: char) -> MovingObject {
        let location = Vector { x: (BOARD_INSIDE as f32) / 2f32, y: (BOARD_INSIDE as f32) / 2f32 };
        let sum = starting_velocity.x.abs() + starting_velocity.y.abs();
        if sum != 0f32 {
            let temp = Vector { x: starting_velocity.x.abs() / sum, y: starting_velocity.y.abs() / sum };
            MovingObject {
                velocity: starting_velocity,
                location,
                symbol,
                ratios: temp,
            }
        } else {
            MovingObject {
                velocity: starting_velocity,
                location,
                symbol,
                ratios: Vector { x: 0.5f32 / sum, y: 0.5f32 },
            }
        }
    }

    fn tick(&mut self, time: f32, final_: bool, collision_velocity: Option<Vector>) {
        if !self.moving() {
            return;
        }
        self.location = Vector {
            x: self.location.x + self.velocity.x * time,
            y: self.location.y + self.velocity.y * time,
        };

        // println!("{} {}", self.velocity.x, self.velocity.y);
        match collision_velocity {
            Some(x) => self.velocity = x,
            None => (),
        }

        if final_ {

            let x_vel;
            let y_vel;
            if self.velocity.x < 0f32 {
                x_vel = 0f32.min(self.velocity.x - DECELERATION * self.ratios.x)
            } else {
                x_vel = 0f32.max(self.velocity.x + DECELERATION * self.ratios.x)
            }
            if self.velocity.y < 0f32 {
                y_vel = 0f32.min(self.velocity.y - DECELERATION * self.ratios.y)
            } else {
                y_vel = 0f32.max(self.velocity.y + DECELERATION * self.ratios.y)
            }
            self.velocity = Vector { x: x_vel, y: y_vel };
        }
    }

    fn get_collisions(self, others: Vec<MovingObject>, offset: usize, max_duration: f32) -> Vec<(usize, f32, usize)> {
        let new_loc = Vector {
            x: self.location.x + self.velocity.x,
            y: self.location.y + self.velocity.y,
        };
        // println!("{} {}", self.velocity.x, self.velocity.y);

        let mut collisions: Vec<(usize, f32, usize)> = Vec::new();

        if new_loc.x < 0f32 && (self.location.x / self.velocity.x).abs() < max_duration {
            collisions.push((127, (self.location.x / self.velocity.x).abs(), offset));
        }
        if new_loc.x > BOARD_INSIDE as f32 &&
            ((BOARD_INSIDE as f32 - self.location.x) / self.velocity.x).abs() < max_duration {
            collisions.push((126, ((BOARD_INSIDE as f32 - self.location.x) / self.velocity.x).abs(), offset));
        }
        if new_loc.y < 0f32 && (self.location.y / self.velocity.y).abs() < max_duration {
            collisions.push((125, (self.location.y / self.velocity.y).abs(), offset));
        }
        if new_loc.y > BOARD_INSIDE as f32 &&
            ((BOARD_INSIDE as f32 - self.location.y) / self.velocity.y).abs() < max_duration {
            collisions.push((124, ((BOARD_INSIDE as f32 - self.location.y) / self.velocity.y).abs(), offset));
        }

        for i in 0..others.len() {
            let other = others[i];
            if !self.moving() && !other.moving() {
                continue;
            }
            let collision_time = find_collision_times(
                self.location, self.velocity, other.location, other.velocity, 1f32,
            );
            if 0f32 < collision_time && collision_time < max_duration {
                collisions.push((offset + 1 + i, collision_time, offset));
            }
        }
        return collisions;
    }

    fn position(self) -> (usize, usize) {
        (self.location.x.round() as usize + 1, self.location.y.round() as usize + 1)
    }

    fn moving(self) -> bool {
        return self.velocity.x != 0f32 || self.velocity.y != 0f32;
    }
}

fn main() -> io::Result<()> {
    // let a = String::from("HEllo");
    clear_board();
    let file = File::open("starts.txt")?;
    let reader = BufReader::new(file);

    let mut tba_objects: Vec<(usize, f32, char)> = Vec::new();

    for line in reader.lines() {
        let temp: String = line.unwrap();
        let split: Vec<&str> = temp.split(" ").collect();
        let wait_time: usize = match split[0].parse::<usize>() {
            Ok(i) => i,
            Err(_) => 0,
        };
        let angle: f32 = match split[1].parse() {
            Ok(i) => i,
            Err(_) => 0f32,
        };
        let symbol: char = match split[2].chars().next() {
            Some(s) => s,
            None => '*'
        };
        tba_objects.push((wait_time, angle, symbol));
    }

    let mut objects: Vec<MovingObject> = Vec::new();
    let mut index = 0;

    while tba_objects[index].0 == 0 {
        let (_, angle, symbol) = tba_objects[index];
        objects.push(MovingObject::new(Vector { x: angle.cos() * 5f32, y: -angle.sin() * 5f32 }, symbol));
        index += 1;
    }

    loop {
        print_board();

        let mut tick_so_far = 0f32;
        while tick_so_far < 1f32 {
            let mut collision_velocities: Vec<Option<Vector>> = vec![None; objects.len()];
            let mut all_collisions: Vec<(usize, f32, usize)> = Vec::new();
            for i in 0..objects.len() {
                let ob = objects[i];
                let temp = ob.get_collisions(objects[(i + 1)..].to_vec(), i, 1f32 - tick_so_far);
                all_collisions.extend(temp);
            }
            let mut duration = 1f32 - tick_so_far;
            let mut used_collisions: Vec<(usize, f32, usize)> = Vec::new();

            if all_collisions.len() != 0 {
                let mut first_collision = 1f32;
                for collision in &all_collisions {
                    let (_, time, _) = collision;
                    first_collision = first_collision.min(*time);
                }
                first_collision = (first_collision * 100f32).ceil() / 100f32;
                duration = first_collision;
                for collision in &all_collisions {
                    let (_, time, _) = collision;
                    if *time <= first_collision {
                        used_collisions.push(*collision);
                    }
                }
            }
            for collision in &used_collisions {
                let (other_idx, _, first_idx) = *collision;
                let mut first_collider = objects[first_idx];

                if other_idx == 126 || other_idx == 127 {
                    collision_velocities[first_idx] = Some(Vector {
                        x: -first_collider.velocity.x,
                        y: first_collider.velocity.y,
                    });
                } else if other_idx == 124 || other_idx == 125 {
                    collision_velocities[first_idx] = Some(Vector {
                        x: first_collider.velocity.x,
                        y: -first_collider.velocity.y,
                    });
                } else {
                    let mut other_collider = objects[other_idx];
                    if other_collider.moving() && first_collider.moving() {
                        let temp = first_collider.ratios;
                        first_collider.ratios = other_collider.ratios;
                        other_collider.ratios = temp;

                        let total_velocity = (
                            (first_collider.velocity.x.powi(2) + first_collider.velocity.y.powi(2)).sqrt() +
                                (other_collider.velocity.x.powi(2) + other_collider.velocity.y.powi(2)).sqrt()
                        ) / 2f32;

                        collision_velocities[other_idx] = Some(Vector {
                            x: total_velocity * other_collider.ratios.x * (if first_collider.velocity.x < 0f32 { -1f32 } else { 1f32 }),
                            y: total_velocity * other_collider.ratios.y * (if first_collider.velocity.y < 0f32 { -1f32 } else { 1f32 }),
                        });

                        collision_velocities[first_idx] = Some(Vector {
                            x: total_velocity * first_collider.ratios.x * (if other_collider.velocity.x < 0f32 { -1f32 } else { 1f32 }),
                            y: total_velocity * first_collider.ratios.y * (if other_collider.velocity.y < 0f32 { -1f32 } else { 1f32 }),
                        });
                    } else if first_collider.moving() {
                        collision_velocities[first_idx] = Some(Vector {
                            x: -first_collider.velocity.x,
                            y: -first_collider.velocity.y,
                        });
                    } else {
                        collision_velocities[other_idx] = Some(Vector {
                            x: -other_collider.velocity.x,
                            y: -other_collider.velocity.y,
                        });
                    }
                }
            }
            for i in 0..objects.len() {
                objects[i].tick(duration, !(tick_so_far + duration < 1f32), collision_velocities[i]);
                let (a, b) = objects[i].position();
                // println!("This shit {} {}", a, b);
            }

            if all_collisions.len() != 0 {
                for collision in used_collisions {
                    let (other_idx, _, first_idx) = collision;
                    let object = objects[first_idx];
                    if other_idx == 127 {
                        unsafe {
                            BOARD[0 + (object.location.y + 1.5f32) as usize * BOARD_WIDTH] = object.symbol;
                        }
                    } else if other_idx == 126 {
                        unsafe {
                            BOARD[BOARD_WIDTH - 1 + (object.location.y + 1.5f32) as usize * BOARD_WIDTH] = object.symbol;
                        }
                    } else if other_idx == 125 {
                        unsafe {
                            BOARD[(object.location.x + 1.5f32) as usize + 0] = object.symbol;
                        }
                    } else if other_idx == 124 {
                        unsafe {
                            BOARD[(object.location.x + 1.5f32) as usize + (BOARD_WIDTH - 1) * BOARD_WIDTH] = object.symbol;
                        }
                    }
                }
            }
            tick_so_far += duration;
        }
        clear_board();

        while index < tba_objects.len() && tba_objects[index].0 == 0 {
            let (_, angle, symbol ) = tba_objects[index];
            objects.push(MovingObject::new(Vector { x: angle.cos() * 5f32, y: -angle.sin() * 5f32 }, symbol));
            index += 1;
        }

        let mut moving: bool = false;
        for object in &objects{
            unsafe {
                let pos = object.position();
                BOARD[pos.0 + pos.1 * BOARD_WIDTH] = object.symbol;
            }
            moving |= object.moving();
        }

        if !moving && (index >= tba_objects.len()) {
            break;
        }
        if index < tba_objects.len() {
            tba_objects[index].0 -= 1;
        }
        println!("{}", objects.len());
    }

    Ok(())
}
