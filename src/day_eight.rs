use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Map {
    inner: HashMap<(Signpost, Direction), Signpost>
}

impl Map {
    fn parse(lines: &[String]) -> Map {
        let mut m = Map {
            inner: HashMap::new(),
        };

        for ele in lines {
            m.parse_line(ele.clone());
        }

        m
    }

    fn parse_line(&mut self, line: String) {
        let parts: Vec<&str> = line.split(" = ").collect();

        let current_signpost = Signpost::parse(parts[0]);

        let go_to_signs: Vec<&str> = parts[1].strip_prefix('(').expect("no (").strip_suffix(')').expect("no )").split(", ").collect();

        self.inner.insert((current_signpost.clone(), Direction::Left), Signpost::parse(go_to_signs[0]));
        self.inner.insert((current_signpost, Direction::Right), Signpost::parse(go_to_signs[1]));
    }
    
    fn navigate(&self, goto: &(Signpost, Direction)) -> Signpost {
        self.inner.get(goto).expect("boo no signpost!").clone()
    }

    fn get_starting_signs(&self) -> Vec<Signpost> {
        self.inner.keys().filter(|x| x.0.is_a_sign()).map(|x| &x.0).cloned().collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right
}

impl Direction {
    fn parse(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("not left or right!"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Signpost {
    inner: String
}

impl Signpost {
    fn parse(val: &str) -> Signpost {
        Signpost { inner: val.to_owned() }
    }

    fn is_a_sign(&self) -> bool {
        self.inner.ends_with('A')
    }

    fn is_z_sign(&self) -> bool {
        self.inner.ends_with('Z')
    }
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo! bad string")).collect();
    let directions: Vec<Direction> = lines[0].chars().map(Direction::parse).collect();

    let map = Map::parse(&lines[2..]);

    let mut current_signpost = Signpost::parse("AAA");
    let mut direction_index = 0;
    let mut num_steps = 0;

    while current_signpost.inner.as_str() != "ZZZ" {
        current_signpost = map.navigate(&(current_signpost, directions[direction_index]));
        direction_index += 1;

        if direction_index >= directions.len() {
            direction_index = 0;
        }

        num_steps += 1;
    }

    println!("steps: {}", num_steps);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo! bad string")).collect();
    let directions: Vec<Direction> = lines[0].chars().map(Direction::parse).collect();

    let map = Map::parse(&lines[2..]);

    let current_signposts = map.get_starting_signs();

        let resolved_signposts: Vec<(Signpost, usize)> = current_signposts.par_iter().map(|signpost| {
            let mut direction_index = 0;
            let mut current_signpost = signpost.clone();
            let mut num_steps: usize = 0;
            while !current_signpost.is_z_sign() {
                current_signpost = map.navigate(&(current_signpost, directions[direction_index]));
                direction_index += 1;
        
                if direction_index >= directions.len() {
                    direction_index = 0;
                }
        
                num_steps += 1;
            }
            (current_signpost, num_steps)
        }).collect();

    let common_steps: Vec<usize> = resolved_signposts.iter().map(|x| x.1).collect();
    let lcm = lcm(&common_steps);

    println!("steps: {}", lcm);

    Ok(())
}

// shamelessly stolen as I am not good at the maths
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
