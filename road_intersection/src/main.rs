use raylib::prelude::*;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
pub struct Car {
    position: Vector2,
    size: Vector2,
    speed: Vector2,
    color: Color,
    life: i32,
    cardinal_direction: CardinalDirection,
    direction: Dir,
    past_light: bool,
}

impl Car {
    pub fn new(
        pos: Vector2,
        size: Vector2,
        speed: Vector2,
        color: Color,
        card: CardinalDirection,
        direction: Dir,
    ) -> Self {
        Self {
            position: pos,
            size,
            speed,
            color,
            life: 1,
            cardinal_direction: card,
            direction,
            past_light: false,
        }
    }
    pub fn match_west(&mut self, dir: Dir, color: Color, light: bool, cars: i32) {
        if !light && self.position.x > (328.0 - (cars * 110) as f32) && !self.past_light {
        } else {
            if self.position.x > 329.0 {
                self.past_light = true;
            }
            match dir {
                Dir::Left => {
                    self.position += self.speed;
                    if self.position.x == SCREEN_WIDTH / 2.0 + 25.0 {
                        self.speed.x = 0.0;
                        self.speed.y = -VELOCITY;
                    }
                    self.color = color;
                }
                Dir::Right => {
                    self.position += self.speed;
                    if self.position.x == SCREEN_WIDTH / 2.0 - ROAD_DIAMETER + 25.0 {
                        self.speed.x = 0.0;
                        self.speed.y = VELOCITY;
                    }
                    self.color = color;
                }
                Dir::Straight => {
                    self.position += self.speed;
                    self.color = color;
                }
            }
        }
    }

    pub fn match_east(&mut self, dir: Dir, color: Color, light: bool, cars: i32) {
        if !light && self.position.x < (621.0 + (cars * 110) as f32) && !self.past_light {
        } else {
            if self.position.x < 621.0 {
                self.past_light = true;
            }
            match dir {
                Dir::Left => {
                    self.position += self.speed;
                    if self.position.x == SCREEN_WIDTH / 2.0 - ROAD_DIAMETER + 25.0 {
                        self.speed.x = 0.0;
                        self.speed.y = VELOCITY;
                    }
                    self.color = color;
                }
                Dir::Right => {
                    self.position += self.speed;
                    if self.position.x == SCREEN_WIDTH / 2.0 + 25.0 {
                        self.speed.x = 0.0;
                        self.speed.y = -VELOCITY;
                    }
                    self.color = color;
                }
                Dir::Straight => {
                    self.position += self.speed;
                    self.color = color;
                }
            }
        }
    }
    pub fn match_north(&mut self, dir: Dir, color: Color, light: bool, cars: i32) {
        if !light && self.position.y < (621.0 + (cars * 110) as f32) && !self.past_light {
        } else {
            if self.position.y < 621.0 {
                self.past_light = true;
            }
            match dir {
                Dir::Left => {
                    self.position += self.speed;
                    if self.position.y == SCREEN_HEIGHT / 2.0 + 25.0 {
                        self.speed.x = VELOCITY;
                        self.speed.y = 0.0;
                    }
                    self.color = color
                }
                Dir::Right => {
                    self.position += self.speed;
                    if self.position.y == SCREEN_HEIGHT / 2.0 - ROAD_DIAMETER + 25.0 {
                        self.speed.x = -VELOCITY;
                        self.speed.y = 0.0;
                    }
                    self.color = color;
                }
                Dir::Straight => {
                    self.position += self.speed;
                    self.color = color;
                }
            }
        }
    }
    pub fn match_south(&mut self, dir: Dir, color: Color, light: bool, cars: i32) {
        if !light && self.position.y > (328.0 - (cars * 110) as f32) && !self.past_light {
        } else {
            if self.position.y > 328.0 {
                self.past_light = true;
            }
            match dir {
                Dir::Left => {
                    self.position += self.speed;
                    if self.position.y == SCREEN_HEIGHT / 2.0 - ROAD_DIAMETER + 25.0 {
                        self.speed.x = -VELOCITY;
                        self.speed.y = 0.0;
                    }
                    self.color = color;
                }
                Dir::Right => {
                    self.position += self.speed;
                    if self.position.y == SCREEN_HEIGHT / 2.0 + 25.0 {
                        self.speed.x = VELOCITY;
                        self.speed.y = 0.0;
                    }
                    self.color = color;
                }
                Dir::Straight => {
                    self.position += self.speed;
                    self.color = color;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Copy, Clone)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("road_intersection")
        .vsync()
        .build();
    // let mut speed = Vector2::new(0.0, 0.0);
    let mut cars = Vec::new();
    let mut north_time = Instant::now();
    let mut south_time = Instant::now();
    let mut west_time = Instant::now();
    let mut east_time = Instant::now();
    let mut light_time = Instant::now();
    let mut vertical_t = false;
    let mut vertical_b = false;
    let mut horizontal_t = false;
    let mut horizontal_b = false;
    while !rl.window_should_close() {
        if light_time.elapsed().as_secs() > 16 {
            horizontal_b = true;
            horizontal_t = false;
            vertical_t = false;
            vertical_b = false;
            if light_time.elapsed().as_secs() > 20 {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = false;
                vertical_b = false;
                light_time = Instant::now();
            }
        } else if light_time.elapsed().as_secs() > 12 {
            if light_time.elapsed().as_secs() > 15 {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = false;
                vertical_b = false;
            } else {
                horizontal_b = false;
                horizontal_t = true;
                vertical_t = false;
                vertical_b = false;
            }
        } else if light_time.elapsed().as_secs() > 8 {
            if light_time.elapsed().as_secs() > 11 {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = false;
                vertical_b = false;
            } else {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = true;
                vertical_b = false;
            }
        } else if light_time.elapsed().as_secs() > 1 {
            if light_time.elapsed().as_secs() > 7 {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = false;
                vertical_b = false;
            } else {
                horizontal_b = false;
                horizontal_t = false;
                vertical_t = false;
                vertical_b = true;
            }
        }
        let mut new_direction = 0;
        if rl.is_key_released(KeyboardKey::KEY_R) { new_direction = get_random_value(1, 4); }
        if rl.is_key_released(KeyboardKey::KEY_RIGHT) { new_direction = 1; }
        if rl.is_key_released(KeyboardKey::KEY_LEFT) { new_direction = 2; }
        if rl.is_key_released(KeyboardKey::KEY_UP) { new_direction = 3; }
        if rl.is_key_released(KeyboardKey::KEY_DOWN) { new_direction = 4; }

        match new_direction {
            1 => {
                if west_time.elapsed().as_millis() < 400 {
                } else {
                    let ran_dir_as_nr = get_random_value(1, 3);
                    let x = match ran_dir_as_nr {
                        1 => (Dir::Left, Color::BLUE),
                        2 => (Dir::Right, Color::RED),
                        3 => (Dir::Straight, Color::PINK),
                        _ => (Dir::Straight, Color::PINK),
                    };
                    // speed
                    cars.push(Car::new(
                        Vector2::new(-130.0, SCREEN_HEIGHT / 2.0 + 25.0),
                        Vector2::new(ROAD_DIAMETER / 2.0, ROAD_DIAMETER / 2.0),
                        Vector2 {
                            x: VELOCITY,
                            y: 0.0,
                        },
                        x.1,
                        CardinalDirection::West,
                        x.0,
                    ));
                    west_time = Instant::now();
                }
            },
            2 => {
                if east_time.elapsed().as_millis() < 400 {
                } else {
                    let ran_dir_as_nr = get_random_value(1, 3);
                    let x = match ran_dir_as_nr {
                        1 => (Dir::Left, Color::BLUE),
                        2 => (Dir::Right, Color::RED),
                        3 => (Dir::Straight, Color::PINK),
                        _ => (Dir::Straight, Color::PINK),
                    };
                    cars.push(Car::new(
                        Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT / 2.0 - ROAD_DIAMETER + 25.0),
                        Vector2::new(ROAD_DIAMETER / 2.0, ROAD_DIAMETER / 2.0),
                        Vector2::new(-VELOCITY, 0.0),
                        x.1,
                        CardinalDirection::East,
                        x.0,
                    ));
                    east_time = Instant::now();
                }
            },
            3 => {
                if north_time.elapsed().as_millis() < 400 {
                } else {
                    let ran_dir_as_nr = get_random_value(1, 3);
                    let x = match ran_dir_as_nr {
                        1 => (Dir::Left, Color::BLUE),
                        2 => (Dir::Right, Color::RED),
                        3 => (Dir::Straight, Color::PINK),
                        _ => (Dir::Straight, Color::PINK),
                    };
                    cars.push(Car::new(
                        Vector2::new(SCREEN_WIDTH / 2.0 + 25.0, SCREEN_HEIGHT),
                        Vector2::new(ROAD_DIAMETER / 2.0, ROAD_DIAMETER / 2.0),
                        Vector2::new(0.0, -VELOCITY),
                        x.1,
                        CardinalDirection::North,
                        x.0,
                    ));
                    north_time = Instant::now();
                }
            },
            4 => {
                if south_time.elapsed().as_millis() < 400 {
                } else {
                    let ran_dir_as_nr = get_random_value(1, 3);
                    let x = match ran_dir_as_nr {
                        1 => (Dir::Left, Color::BLUE),
                        2 => (Dir::Right, Color::RED),
                        3 => (Dir::Straight, Color::PINK),
                        _ => (Dir::Straight, Color::PINK),
                    };
                    cars.push(Car::new(
                        Vector2::new(SCREEN_WIDTH / 2.0 - ROAD_DIAMETER + 25.0, -130.0),
                        Vector2::new(ROAD_DIAMETER / 2.0, ROAD_DIAMETER / 2.0),
                        Vector2::new(0.0, VELOCITY),
                        x.1,
                        CardinalDirection::South,
                        x.0,
                    ));
                    south_time = Instant::now();
                }
            },
            _ => ()
        }
        let mut north_cars = 0;
        let mut south_cars = 0;
        let mut east_cars = 0;
        let mut west_cars = 0;
        for car in cars.iter_mut() {
            match car.cardinal_direction {
                CardinalDirection::West => {
                    car.match_west(car.direction, car.color, horizontal_b, west_cars);
                    if !car.past_light {
                        west_cars += 1;
                    }
                }
                CardinalDirection::East => {
                    car.match_east(car.direction, car.color, horizontal_t, east_cars);
                    if !car.past_light {
                        east_cars += 1;
                    }
                }
                CardinalDirection::South => {
                    car.match_south(car.direction, car.color, vertical_b, south_cars);
                    if !car.past_light {
                        south_cars += 1;
                    }
                }
                CardinalDirection::North => {
                    car.match_north(car.direction, car.color, vertical_t, north_cars);
                    if !car.past_light {
                        north_cars += 1;
                    }
                }
            }
            if car.position.x > SCREEN_WIDTH
                || car.position.x < -140.0
                || car.position.y > SCREEN_HEIGHT
                || car.position.y < -140.0
            {
                car.life -= 1;
            }
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGREEN);
        let mut traffic_light_col_vt = Color::GREEN;
        let mut traffic_light_col_ht = Color::GREEN;
        let mut traffic_light_col_vb = Color::GREEN;
        let mut traffic_light_col_hb = Color::GREEN;
        if vertical_b {
            traffic_light_col_vb = Color::GREEN;
            traffic_light_col_vt = Color::RED;
            traffic_light_col_ht = Color::RED;
            traffic_light_col_hb = Color::RED;
        } else if vertical_t {
            traffic_light_col_vb = Color::RED;
            traffic_light_col_vt = Color::GREEN;
            traffic_light_col_ht = Color::RED;
            traffic_light_col_hb = Color::RED;
        } else if horizontal_b {
            traffic_light_col_vb = Color::RED;
            traffic_light_col_vt = Color::RED;
            traffic_light_col_ht = Color::RED;
            traffic_light_col_hb = Color::GREEN;
        } else if horizontal_t {
            traffic_light_col_vb = Color::RED;
            traffic_light_col_vt = Color::RED;
            traffic_light_col_ht = Color::GREEN;
            traffic_light_col_hb = Color::RED;
        } else {
            traffic_light_col_vb = Color::RED;
            traffic_light_col_vt = Color::RED;
            traffic_light_col_ht = Color::RED;
            traffic_light_col_hb = Color::RED;
        }

        d.draw_rectangle(
            0,
            YPOS_ROAD1,
            SCREEN_WIDTH as i32,
            ROAD_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_rectangle(
            0,
            POSY_ROAD as i32,
            SCREEN_WIDTH as i32,
            ROAD_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_rectangle(
            XPOS_ROAD1,
            0,
            ROAD_DIAMETER as i32,
            SCREEN_HEIGHT as i32,
            Color::BLACK,
        );
        d.draw_rectangle(
            POSX_ROAD as i32,
            0,
            ROAD_DIAMETER as i32,
            SCREEN_HEIGHT as i32,
            Color::BLACK,
        );
        d.draw_rectangle_lines(
            0,
            YPOS_ROAD1,
            SCREEN_WIDTH as i32,
            ROAD_DIAMETER as i32,
            Color::WHITE,
        );
        d.draw_rectangle_lines(
            0,
            POSY_ROAD as i32,
            SCREEN_WIDTH as i32,
            ROAD_DIAMETER as i32,
            Color::WHITE,
        );
        d.draw_rectangle_lines(
            XPOS_ROAD1,
            0,
            ROAD_DIAMETER as i32,
            SCREEN_HEIGHT as i32,
            Color::WHITE,
        );
        d.draw_rectangle_lines(
            POSX_ROAD as i32,
            0,
            ROAD_DIAMETER as i32,
            SCREEN_HEIGHT as i32,
            Color::WHITE,
        );
        d.draw_rectangle(
            XPOS_TRAF1,
            YPOS_TRAF1,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_circle(XPOS_TRAF1 + 25, YPOS_TRAF1 + 25, 15.0, traffic_light_col_vb);
        d.draw_rectangle(
            XPOS_TRAF2,
            YPOS_TRAF2,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_circle(XPOS_TRAF2 + 25, YPOS_TRAF2 + 25, 15.0, traffic_light_col_vt);
        d.draw_rectangle(
            XPOS_TRAF2,
            YPOS_TRAF1,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_circle(XPOS_TRAF2 + 25, YPOS_TRAF1 + 25, 15.0, traffic_light_col_ht);
        d.draw_rectangle(
            XPOS_TRAF1,
            YPOS_TRAF2,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::BLACK,
        );
        d.draw_circle(XPOS_TRAF1 + 25, YPOS_TRAF2 + 25, 15.0, traffic_light_col_hb);
        d.draw_rectangle_lines(
            XPOS_TRAF1,
            YPOS_TRAF1,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::RED,
        );
        d.draw_rectangle_lines(
            XPOS_TRAF2,
            YPOS_TRAF2,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::RED,
        );
        d.draw_rectangle_lines(
            XPOS_TRAF2,
            YPOS_TRAF1,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::RED,
        );
        d.draw_rectangle_lines(
            XPOS_TRAF1,
            YPOS_TRAF2,
            TRAFFIC_DIAMETER as i32,
            TRAFFIC_DIAMETER as i32,
            Color::RED,
        );
        d.draw_fps(20, 20);
        for car in cars.iter() {
            d.draw_rectangle_v(car.position, car.size, car.color);
        }

        cars.retain(|car| car.life > 0);
    }
}

const SCREEN_WIDTH: f32 = 1000.0;

const SCREEN_HEIGHT: f32 = 1000.0;

const ROAD_DIAMETER: f32 = SCREEN_HEIGHT / 10.0;
const TRAFFIC_DIAMETER: f32 = 50.0;
const POSX_ROAD: f32 = SCREEN_WIDTH / 2.0;

const POSY_ROAD: f32 = SCREEN_HEIGHT / 2.0;
const VELOCITY: f32 = 5.0;
const YPOS_ROAD1: i32 = (POSY_ROAD - ROAD_DIAMETER) as i32;
const XPOS_ROAD1: i32 = (POSX_ROAD - ROAD_DIAMETER) as i32;
const XPOS_TRAF1: i32 = (POSX_ROAD - ROAD_DIAMETER - TRAFFIC_DIAMETER - 10.0) as i32;
const YPOS_TRAF1: i32 = (POSY_ROAD - ROAD_DIAMETER - TRAFFIC_DIAMETER - 10.0) as i32;
const XPOS_TRAF2: i32 = (POSX_ROAD + ROAD_DIAMETER + 10.0) as i32;
const YPOS_TRAF2: i32 = (POSY_ROAD + ROAD_DIAMETER + 10.0) as i32;