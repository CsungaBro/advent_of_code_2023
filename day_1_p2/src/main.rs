use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Numbers {
    first: u32,
    last: u32,
}

impl Numbers {
    fn new() -> Self {
        Numbers { first: 0, last: 0 }
    }
    fn get_number(self: Self) -> usize {
        let num_string = format!("{}{}", self.first, self.last);
        num_string.parse::<usize>().unwrap()
    }
    fn get_first(self: &Self, line: String, map: &HashMap<&str, u32>) -> u32 {
        let new_line = line.clone();
        let mut container = String::new();

        for c in new_line.chars() {
            let m = c.to_digit(10);
            match m {
                Some(num) => return num,
                None => container.push(c),
            }
            for key in map.keys() {
                if container.contains(key) {
                    return *map.get(key).unwrap();
                }
            }
        }
        return 0;
    }

    fn get_last(self: &Self, line: String, map: &HashMap<&str, u32>) -> u32 {
        let new_line = line.clone();
        let mut container = String::new();

        for c in new_line.chars().rev() {
            let m = c.to_digit(10);
            match m {
                Some(num) => return num,
                None => container.insert(0, c),
            }
            for key in map.keys() {
                if container.contains(key) {
                    return *map.get(key).unwrap();
                }
            }
        }
        return 0;
    }
    fn from_list(line: String, map: &HashMap<&str, u32>) -> Self {
        let mut numbers = Numbers::new();
        numbers.first = numbers.get_first(line.clone(), map);
        numbers.last = numbers.get_last(line.clone(), map);
        numbers
    }
}

fn convert_string_nums(line: String, map: &HashMap<&str, &str>) -> String {
    let mut new_data = line.clone();
    println!("{new_data}");

    let mut container = String::new();
    for c in line.chars() {
        container.push(c);
        for key in map.keys() {
            if container.contains(key) {
                new_data = new_data.replace(key, map.get(key).unwrap());
                container = String::new();
                //container.pop()
                println!("{new_data}");
            }
        }
    }

    println!("");

    return new_data;
}

fn get_digit_sum(file_path: &Path) -> usize {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let string_nums: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let numbers: Vec<Numbers> = contents
        .lines()
        .map(|lines| Numbers::from_list(lines.to_string(), &string_nums))
        .collect();
    println!("{:?}", numbers);

    let nums: Vec<usize> = numbers
        .into_iter()
        .map(|num_struct| num_struct.get_number())
        .collect();
    println!("{:?}", nums);
    nums.into_iter().sum::<usize>()
}

fn main() {
    let file_path = Path::new("./data/data.txt");
    let digit = get_digit_sum(file_path);
    println!("{digit}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit_sum_1() {
        let test_path = Path::new("./data/test_2.txt");
        let digit = get_digit_sum(&test_path);
        assert_eq!(digit, 281);
    }

    //#[test]
    fn test_get_digit_sum_2() {
        let test_path = Path::new("./data/data.txt");
        let digit = get_digit_sum(&test_path);
        assert_eq!(digit, 1);
    }
}
