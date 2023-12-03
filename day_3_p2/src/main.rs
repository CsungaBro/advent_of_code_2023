use log::{debug, error, info, warn};
use std::collections::btree_map::Values;
use std::fs;
use std::path::Path;

//#[derive(Debug, Clone, Default, Copy)]
#[derive(Debug, Clone, Default)]
struct Coordinates {
    x_start: u32,
    x_end: u32,
    y: u32,
    first_set: bool,
}

impl Coordinates {
    fn new() -> Self {
        Coordinates {
            x_start: 0,
            x_end: 0,
            y: 0,
            first_set: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Gear {
    x: u32,
    y: u32,
    value: u32,
}

#[derive(Debug, Clone, Default)]
struct Part {
    value: u32,
    coords: Coordinates,
}

impl Part {
    fn new() -> Self {
        Part {
            value: 0,
            coords: Coordinates::new(),
        }
    }

    fn add_coords(&mut self, x: u32, y: u32) {
        if !self.coords.first_set {
            self.coords.x_start = x;
            self.coords.y = y;
            self.coords.first_set = true;
        }
        self.coords.x_end = x;
    }

    fn append(&mut self, num: u32) {
        self.value = self.value * 10 + num;
    }

    fn _from(value: u32, x_start: u32, x_end: u32, y: u32) -> Self {
        Part {
            value,
            coords: Coordinates {
                x_start,
                x_end,
                y,
                first_set: true,
            },
        }
    }

    fn is_gear(&self, map: &Vec<Vec<char>>) -> Option<Gear> {
        let y_coord = self.coords.y as i32;
        let x_start = self.coords.x_start as i32;
        let x_end = self.coords.x_end as i32;

        let x_coords = x_start..x_end + 1;

        let y_range = y_coord - 1..y_coord + 2;
        let x_range = x_start - 1..x_end + 2;

        let y_limit = map.len() as i32;
        let x_limit = map[0].len() as i32;

        for y in y_range {
            for x in x_range.clone() {
                if y < 0 || x < 0 {
                } else if y >= y_limit || x >= x_limit {
                } else if y == y_coord && x_coords.contains(&x) {
                    debug!(
                        "Word:   {}, X:{}  Y:{}, value:{}",
                        map[y as usize][x as usize], x, y, self.value
                    );
                } else if map[y as usize][x as usize] == '*' {
                    info!(
                        "Gear:    {}, X:{}  Y:{}, value:{}",
                        map[y as usize][x as usize], x, y, self.value
                    );
                    return Some(Gear {
                        x: x as u32,
                        y: y as u32,
                        value: self.value,
                    });
                }
            }
        }
        None
    }
}

fn get_result(file_path: &Path) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let container: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    debug!("{:?}", container);

    let mut parts: Vec<Part> = vec![];
    for (y, line) in container.iter().enumerate() {
        let mut part = Part::new();

        for (x, ch) in line.into_iter().enumerate() {
            let new_ch = ch.to_digit(10);
            match new_ch {
                Some(num) => {
                    part.append(num);
                    part.add_coords(x as u32, y as u32);
                }
                None => {
                    if part.value != 0 {
                        debug!("{:?}", part);
                        parts.push(part);
                        part = Part::new();
                    }
                }
            }
        }
        if part.value != 0 {
            debug!("{:?}", part);
            parts.push(part);
        }
    }
    let mut gears: Vec<Gear> = parts
        .into_iter()
        .filter_map(|part| part.is_gear(&container))
        .collect();
    info!("{:?}", gears);

    let mut sum = 0;
    for (count_1, gear_1) in gears.clone().into_iter().enumerate() {
        if count_1 == gears.len() - 1 {
            break;
        }
        for (_count_2, gear_2) in gears[count_1 + 1..].into_iter().enumerate() {
            if gear_1.x == gear_2.x && gear_1.y == gear_2.y {
                sum += gear_1.value * gear_2.value;
                //gears.remove(count_1);
                //gears.remove(count_2);
                info!("\nGear 1: {:?}\nGear 2: {:?}", gear_1, gear_2);
                break;
            }
        }
    }

    sum
}

fn main() {
    env_logger::init();
    let file_path = Path::new("./data/data.txt");
    let result = get_result(&file_path);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_sum_1() {
        env_logger::init();
        let test_path = Path::new("./data/test.txt");
        let result = get_result(&test_path);
        assert_eq!(result, 467835);
    }
}
