use std::{io::{BufReader, BufRead}, fs::File};

struct Game {
    game_id: i64,
    subgames: Vec<SubGame>,
}

struct SubGame {
    blue: i64,
    red: i64,
    green: i64,
}

fn build_game(line: &str) -> Game {
    let chunks = line.split(':').collect::<Vec<&str>>();

    let game_id = str::parse::<i64>(chunks[0].split(' ').collect::<Vec<&str>>()[1]).expect("failed to extract game id");
    let mut subgames: Vec<SubGame> = Vec::new();
    for subgame in chunks[1].split(';') {
        subgames.push(build_subgame(subgame));
    }

    Game { game_id, subgames }
}

fn build_subgame(subgame: &str) -> SubGame {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for cube_set in subgame.split(',') {
        let (b, r, g) = parse_cube_set(cube_set);
        blue += b;
        red += r;
        green += g;
    }

    SubGame { blue, red, green }
}

fn parse_cube_set(cube_set: &str) -> (i64, i64, i64) {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    let cube_set = match cube_set.strip_prefix(' ') {
        Some(fixed) => fixed,
        None => cube_set,
    };
    let cubes = cube_set.split(' ').collect::<Vec<&str>>();
    let cube_count = str::parse::<i64>(cubes[0]).expect("failed to extract cube count id");
    let cube_name = cubes[1];

    match cube_name {
        "blue" => blue += cube_count,
        "green" => green += cube_count,
        "red" => red += cube_count,
        _ => panic!("bad case")
    };

    (blue, red, green)
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {

    let mut games: Vec<Game> = Vec::new();

    for line in in_file.lines() {
        let validated_line = line?;
        games.push(build_game(&validated_line));
    }

    let result: i64 = games.iter().map(|x| {
        if x.subgames.iter().filter(|x| x.blue > 14 || x.red > 12 || x.green > 13).count() > 0 {
            0
        } else {
            x.game_id
        }
    }).sum();

    print!("result: {}", result);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let mut games: Vec<Game> = Vec::new();

    for line in in_file.lines() {
        let validated_line = line?;
        games.push(build_game(&validated_line));
    }

    let result: i64 = games.iter().map(|x| {
        let min_cubes = x.subgames.iter().fold((0, 0, 0), |mut acc, subgame| {
            if acc.0 < subgame.blue {
                acc.0 = subgame.blue;
            }

            if acc.1 < subgame.green {
                acc.1 = subgame.green;
            }

            if acc.2 < subgame.red {
                acc.2 = subgame.red;
            }

            acc
        });

        min_cubes.0 * min_cubes.1 * min_cubes.2
    }).sum();

    print!("result: {}", result);

    Ok(())
}