pub const BOARD_WIDTH: usize = 16;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

const BOARD_INSIDE: usize = BOARD_WIDTH - 3;

pub static mut BOARD: [char; BOARD_SIZE] = ['X'; BOARD_SIZE];

const DECELERATION: f32 = -0.08f32;

pub const MAXIMUM_OBJECTS: usize = 10;
const MAX_COLLISIONS_PER_OBJECT: usize = 10;
const MAXIMUM_COLLISIONS: usize = MAXIMUM_OBJECTS * MAX_COLLISIONS_PER_OBJECT;

pub fn clear_board() {
    for y in 1..(BOARD_WIDTH - 1) {
        for x in 1..(BOARD_WIDTH - 1) {
            unsafe {
                BOARD[x + y * BOARD_WIDTH] = '.';
            }
        }
    }
}

pub fn print_board() {
    for y in 0..(BOARD_WIDTH) {
        for x in 0..(BOARD_WIDTH) {
            //unsafe {
            //}
        }
    }
}


fn solve_quadratic(a: f32, b: f32, c: f32) -> f32 {
    let determinant: f32 = b * b - 4f32 * a * c;
    // Since we only care about collisions that happen between t=[0, 1)
    // it is ok to return -1 here
    if determinant < 0f32 {
        -1f32
    } else {
        (-b - fast_sqrt(determinant)) / (2f32 * a)
    }
}

fn find_collision_times(start1: Vector, v1: Vector, start2: Vector, v2: Vector, radius: f32) -> f32 {
    let a: f32 = pow2(v1.x - v2.x) + pow2(v1.y - v2.y);

    let b: f32 = -2f32 * (v1.x - v2.x) * (start2.x - start1.x)
        - 2f32 * (v1.y - v2.y) * (start2.y - start1.y);

    let c: f32 = pow2(start1.x - start2.x) +
        pow2(start1.y - start2.y) - pow2(radius);

    if abs(a) < f32::EPSILON {
        return -c / b;
    }
    if abs(a) < f32::EPSILON && abs(b) < f32::EPSILON {
        return -1f32;
    }
    solve_quadratic(a, b, c)
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32,
}


#[derive(Debug, Clone)]
pub struct MovingObject {
    velocity: Vector,
    location: Vector,
    ratios: Vector,
    pub(crate) symbol: char,
    age: usize,
}

impl MovingObject {
    pub fn new(starting_location: Vector, starting_velocity: Vector, symbol: char) -> MovingObject {
        let location = starting_location;
        let sum = abs(starting_velocity.x) + abs(starting_velocity.y);
        if sum != 0f32 {
            let temp = Vector { x: abs(starting_velocity.x) / sum, y: abs(starting_velocity.y) / sum };
            MovingObject {
                velocity: starting_velocity,
                location,
                symbol,
                ratios: temp,
                age: 0,
            }
        } else {
            MovingObject {
                velocity: starting_velocity,
                location,
                symbol,
                ratios: Vector { x: 0.5f32 / sum, y: 0.5f32 },
                age: 0,
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
                x_vel = min(0f32, self.velocity.x - DECELERATION * self.ratios.x)
            } else {
                x_vel = max(0f32, self.velocity.x + DECELERATION * self.ratios.x)
            }
            if self.velocity.y < 0f32 {
                y_vel = min(0f32, self.velocity.y - DECELERATION * self.ratios.y)
            } else {
                y_vel = max(0f32, self.velocity.y + DECELERATION * self.ratios.y)
            }
            self.velocity = Vector { x: x_vel, y: y_vel };
        }
    }

    fn get_collisions(&self,
                      others: &[Option<MovingObject>; MAXIMUM_OBJECTS],
                      offset: usize,
                      max_duration: f32, ) -> [Option<(usize, f32, usize)>; MAX_COLLISIONS_PER_OBJECT] {
        let new_loc = Vector {
            x: self.location.x + self.velocity.x,
            y: self.location.y + self.velocity.y,
        };
        // println!("{} {}", self.velocity.x, self.velocity.y);

        let mut num_collisions: usize = 0;
        let mut collisions: [Option<(usize, f32, usize)>; MAX_COLLISIONS_PER_OBJECT] = [None; MAX_COLLISIONS_PER_OBJECT];

        if new_loc.x < 0f32 && abs(self.location.x / self.velocity.x) < max_duration {
            collisions[num_collisions] = Some((127, abs(self.location.x / self.velocity.x), offset));
            num_collisions += 1;
        }
        if new_loc.x > BOARD_INSIDE as f32 &&
            abs((BOARD_INSIDE as f32 - self.location.x) / self.velocity.x) < max_duration {
            collisions[num_collisions] = Some((126, abs((BOARD_INSIDE as f32 - self.location.x) / self.velocity.x), offset));
            num_collisions += 1;
        }
        if new_loc.y < 0f32 && abs(self.location.y / self.velocity.y) < max_duration {
            collisions[num_collisions] = Some((125, abs(self.location.y / self.velocity.y), offset));
            num_collisions += 1;
        }
        if new_loc.y > BOARD_INSIDE as f32 &&
            abs((BOARD_INSIDE as f32 - self.location.y) / self.velocity.y) < max_duration {
            collisions[num_collisions] = Some((124, abs((BOARD_INSIDE as f32 - self.location.y) / self.velocity.y), offset));
            num_collisions += 1;
        }

        for i in offset + 1..MAXIMUM_OBJECTS {
            let other = match &others[i] {
                Some(o) => o,
                None => continue
            };
            if !self.moving() && !other.moving() {
                continue;
            }
            let collision_time = find_collision_times(
                self.location, self.velocity, other.location, other.velocity, 1f32,
            );
            if 0f32 < collision_time && collision_time < max_duration {
                collisions[num_collisions] = Some((i, collision_time, offset));
                num_collisions += 1;
            }
        }
        return collisions;
    }

    pub fn position(&self) -> (usize, usize) {
        ((self.location.x + 1.5) as usize, (self.location.y + 1.5) as usize)
    }

    pub fn moving(&self) -> bool {
        return self.velocity.x != 0f32 || self.velocity.y != 0f32;
    }
    pub fn clear_symbol(&mut self)  {
        self.symbol = '.';
    }
    pub fn add_age(&mut self)  {
        self.age = self.age + 1;
    }
    pub fn get_age(&self) -> usize {
        self.age
    }
}

pub fn game_tick(objects: &mut [Option<MovingObject>; 10], number_of_objects: usize) {
    let mut tick_so_far = 0f32;
    while tick_so_far < 1f32 {
        let mut collision_velocities: [Option<Vector>; MAXIMUM_OBJECTS] = [None; MAXIMUM_OBJECTS];
        let mut all_collisions: [Option<(usize, f32, usize)>; MAXIMUM_COLLISIONS] = [None; MAXIMUM_COLLISIONS];
        let mut total_collisions: usize = 0;
        for i in 0..number_of_objects {
            let ob = match &objects[i] {
                Some(o) => o,
                None => continue,
            };
            let temp = ob.get_collisions(objects, i, 1f32 - tick_so_far);
            for i in 0..MAX_COLLISIONS_PER_OBJECT {
                match temp[i] {
                    Some(_) => (),
                    None => break
                };
                all_collisions[total_collisions] = temp[i];
                total_collisions += 1;
            }
        }
        let mut duration = 1f32 - tick_so_far;

        let mut num_used_collisions = 0;
        let mut used_collisions: [Option<(usize, f32, usize)>; MAXIMUM_COLLISIONS] = [None; MAXIMUM_COLLISIONS];

        if total_collisions != 0 {
            let mut first_collision = 1f32;

            for collision in &all_collisions {
                let time = match collision {
                    Some(t) => t.1,
                    None => break
                };
                first_collision = min(first_collision, time);
            }

            first_collision = (first_collision * 100f32 + 1f32) / 100f32;
            duration = first_collision;

            for collision in &all_collisions {
                let time = match collision {
                    Some(t) => t.1,
                    None => break
                };
                if time <= first_collision {
                    used_collisions[num_used_collisions] = *collision;
                    num_used_collisions += 1;
                }
            }
        }
        for collision in &used_collisions {
            let (other_idx, first_idx) = match collision {
                Some(o) => (o.0, o.2),
                None => break,
            };
            let mut first_collider = match objects[first_idx].clone() {
                Some(o) => o,
                None => panic!()
            };

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
                let other_collider = &mut match objects[other_idx].clone() {
                    Some(o) => o,
                    None => panic!()
                };
                if other_collider.moving() && first_collider.moving() {
                    let temp = first_collider.ratios;
                    first_collider.ratios = other_collider.ratios;
                    other_collider.ratios = temp;

                    let total_velocity = (
                        fast_sqrt(pow2(first_collider.velocity.x) + pow2(first_collider.velocity.y)) +
                            fast_sqrt(pow2(other_collider.velocity.x) + pow2(other_collider.velocity.y))
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
        for i in 0..number_of_objects {
            let temp = &mut match objects[i].clone() {
                Some(o) => o,
                None => continue
            };
            temp.tick(duration,
                      !(tick_so_far + duration < 1f32),
                      collision_velocities[i]);

        }

        if num_used_collisions != 0 {
            for collision in &used_collisions {
                let (other_idx, first_idx) = match collision {
                    Some(o) => (o.0, o.2),
                    None => break,
                };
                let object = match &objects[first_idx] {
                    Some(o) => o,
                    None => panic!()
                };
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
}


#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

pub fn fast_sqrt(num: f32) -> f32 {
    let x2 = num * 0.5f32;
    let threehalfs = 1.5f32;

    let mut conv = MyUnion { f2: num };
    unsafe {
        conv.f1 = 0x5f3759df - (conv.f1 >> 1);

        conv.f2 *= threehalfs - (x2 * conv.f2 * conv.f2);
        return 1f32 / conv.f2;
    }
}

pub fn abs(num: f32) -> f32 {
    let mut conv = MyUnion { f2: num };
    unsafe {
        conv.f1 &= 0x7fffffff;
        return conv.f2;
    }
}

pub fn pow2(num: f32) -> f32 {
    num * num
}

fn min(first: f32, second: f32) -> f32 {
    if first < second { first } else { second }
}

fn max(first: f32, second: f32) -> f32 {
    if first > second { first } else { second }
}