use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl Round {
    fn is_possible(self: &Self, cubes: &Cubes) -> bool {
        if self.red > cubes.red {
            return false;
        }
        if self.green > cubes.green {
            return false;
        }
        if self.blue > cubes.blue {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
impl Game {
    fn is_possible(self: &Self, cubes: &Cubes) -> bool {
        for round in self.rounds.iter() {
            if !round.is_possible(cubes) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Play {
    games: Vec<Game>,
    cubes: Cubes,
}
impl Play {
    fn get_possible_games(self: &Self) -> Vec<Game> {
        let mut container: Vec<Game> = vec![];
        for game in self.games.iter() {
            if game.is_possible(&self.cubes) {
                container.push(game.clone());
            }
        }

        container
    }
    fn from_list_line(lines: Vec<&str>) -> Self {
        let max_cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut played_games: Vec<Game> = vec![];
        for line in lines {
            let data = line.split(": ").collect::<Vec<&str>>();
            //println!("{:?}", data);

            let game_id = data[0]
                .strip_prefix("Game ")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            //println!("{:?}", game_id);

            let games = data[1].split("; ").collect::<Vec<&str>>();
            let mut rounds: Vec<Round> = vec![];
            for game in games {
                let cubes = game.split(", ").collect::<Vec<&str>>();
                //println!("{:?}", cubes);
                let mut round = Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for cube in cubes {
                    if cube.contains("red") {
                        round.red = cube.strip_suffix(" red").unwrap().parse::<u32>().unwrap();
                    }
                    if cube.contains("green") {
                        round.green = cube.strip_suffix(" green").unwrap().parse::<u32>().unwrap();
                    }
                    if cube.contains("blue") {
                        round.blue = cube.strip_suffix(" blue").unwrap().parse::<u32>().unwrap();
                    }
                }
                //println!("{:?}", round);
                rounds.push(round);
            }
            let game = Game {
                id: game_id,
                rounds,
            };
            //println!("{:?}", game);
            played_games.push(game);
        }
        Play {
            games: played_games,
            cubes: max_cubes,
        }
    }
}
fn get_ids_sum(file_path: &Path) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let play = Play::from_list_line(contents.lines().collect::<Vec<&str>>());
    let possible_games = play.get_possible_games();

    println!("{:?}", possible_games);

    let sum_ids = possible_games.into_iter().map(|game| game.id).sum();

    sum_ids
}

fn main() {
    let file_path = Path::new("./data/data.txt");
    let ids = get_ids_sum(&file_path);
    println!("{ids}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ids_sum_1() {
        let test_path = Path::new("./data/test.txt");
        let ids = get_ids_sum(&test_path);
        assert_eq!(ids, 8);
    }

    //#[test]
    fn test_get_ids_sum_2() {
        let test_path = Path::new("./data/data.txt");
        let ids = get_ids_sum(&test_path);
        assert_eq!(ids, 55386);
    }
}
