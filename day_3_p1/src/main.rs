use log::{debug, error, info, warn};
use std::fs;
use std::path::Path;

//#[derive(Debug, Clone, Default, Copy)]
#[derive(Debug, Clone, Default)]
struct Coordinates {
    x: i32,
    y: i32,
    first_set: bool,
}

impl Coordinates {
    fn new() -> Self {
        Coordinates {
            x: 0,
            y: 0,
            first_set: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Part {
    value: u32,
    coords: Coordinates,
    len: i32,
}

impl Part {
    fn new() -> Self {
        Part {
            value: 0,
            coords: Coordinates::new(),
            len: 0,
        }
    }

    fn add_coords(&mut self, x: i32, y: i32) {
        if !self.coords.first_set {
            self.coords.x = x;
            self.coords.y = y;
            self.coords.first_set = true;
        }
        self.len += 1;
    }

    fn append(&mut self, num: u32) {
        self.value = self.value * 10 + num;
    }

    fn is_part(&self, map: &Vec<Vec<char>>) -> bool {
        let num_in_x = self.coords.x..self.coords.x + self.len;

        let x_range = self.coords.x - 1..self.coords.x + self.len + 1;
        let y_range = self.coords.y - 1..self.coords.y + 2;

        let y_limit = map.len() as i32;
        let x_limit = map[0].len() as i32;

        for y in y_range {
            for x in x_range.clone() {
                if y < 0 || x < 0 {
                } else if y >= y_limit || x >= x_limit {
                } else if y == self.coords.y && num_in_x.contains(&x) {
                    debug!(
                        "Word:   {}, X:{}  Y:{}, value:{}",
                        map[y as usize][x as usize], x, y, self.value
                    );
                } else if map[y as usize][x as usize] != '.' {
                    info!(
                        "In:     {}, X:{}  Y:{}, value:{}",
                        map[y as usize][x as usize], x, y, self.value
                    );
                    return true;
                } else {
                    debug!(
                        "Not in: {}, X:{}  Y:{}, value:{}",
                        map[y as usize][x as usize], x, y, self.value
                    );
                }
            }
        }
        warn!("************************************");
        warn!("The number: {} is not a part number", self.value);
        warn!("************************************");

        false
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
                    part.add_coords(x as i32, y as i32);
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

    let true_parts: Vec<Part> = parts
        .into_iter()
        .filter(|part| part.is_part(&container))
        .collect();

    debug!("{:?}", true_parts);

    let sum_part_nums = true_parts.into_iter().map(|part| part.value).sum();
    sum_part_nums
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_get_result_sum_11() {
        let test_path = Path::new("./data/test_2.txt");
        let result = get_result(&test_path);
        assert_eq!(result, 4419);
    }

    #[test]
    fn test_get_result_sum_2() {
        let test_path = Path::new("./data/data.txt");
        let result = get_result(&test_path);
        assert_eq!(result, 539637);
    }
}
