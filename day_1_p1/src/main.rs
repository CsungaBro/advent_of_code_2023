use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Numbers {
    first: u32,
    last: u32,
    last_found: u32,
    found_first: bool,
}
impl Numbers {
    fn new() -> Self {
        Numbers {
            first: 0,
            last: 0,
            last_found: 0,
            found_first: false,
        }
    }
    fn get_number(self: Self) -> usize {
        let num_string = format!("{}{}", self.first, self.last);
        num_string.parse::<usize>().unwrap()
    }
    fn from_list(list: Vec<u32>) -> Self {
        let mut numbers = Numbers::new();
        for num in list {
            if !numbers.found_first {
                numbers.first = num;
                numbers.last = num;
                numbers.found_first = true;
            }
            numbers.last_found = num;
        }
        numbers.last = numbers.last_found;
        numbers
    }
}

fn get_digit_sum(file_path: &Path) -> usize {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let num_container: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    let numbers: Vec<Numbers> = num_container
        .into_iter()
        .map(|nums| Numbers::from_list(nums))
        .collect();
    let nums: Vec<usize> = numbers
        .into_iter()
        .map(|num_struct| num_struct.get_number())
        .collect();
    println!("{:?}", nums);
    nums.into_iter().sum::<usize>()
}

fn main() {
    let file_path = Path::new("./data/input_1.txt");
    let digit = get_digit_sum(file_path);
    println!("{digit}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit_sum_1() {
        let test_path = Path::new("./data/test_1.txt");
        let digit = get_digit_sum(&test_path);
        assert_eq!(digit, 142);
    }

    #[test]
    fn test_get_digit_sum_2() {
        let test_path = Path::new("./data/input_1.txt");
        let digit = get_digit_sum(&test_path);
        assert_eq!(digit, 55386);
    }
}
