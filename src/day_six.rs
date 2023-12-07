use std::{io::{BufReader, BufRead}, fs::File};

struct RaceData {
    inner: Vec<(usize, usize)>
}

impl RaceData {
    fn parse(lines: Vec<String>) -> RaceData {
        let mut race_map = Vec::new();
        
        // parse time line

        let times_str = lines[0].strip_prefix("Time:        ").expect("boo this isn't a time entry");
        let times: Vec<usize> = times_str.split("     ").map(|x| str::parse(x).expect("boo this isn't a valid usize")).collect();
        let distances_str = lines[1].strip_prefix("Distance:   ").expect("boo this isn't a distance entry");
        let distances: Vec<usize> = distances_str.split("   ").map(|x| str::parse(x).expect("boo this isn't a valid usize")).collect();

        for (idx, time) in times.iter().enumerate() {
            race_map.push((*time, distances[idx]));
        }


        RaceData { inner: race_map }
    }
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let rm = RaceData::parse(in_file.lines().map(|x| x.expect("boo bad string")).collect());

    let mut number_of_ways = Vec::new();
    for (time, distance) in rm.inner {
        let mut button_times = Vec::new();
        for vel in 1..time {
            // compute distance
            // velocity = vel
            // t = time - vel
            let distance_travelled = vel * (time - vel);
            if distance_travelled > distance {
                button_times.push(vel);
            }
        }
        number_of_ways.push(button_times.len());
    }

    let result: usize = number_of_ways.iter().product();

    println!("result: {}", result);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo bad string")).collect();
    let times_str = lines[0].strip_prefix("Time:        ").expect("boo this isn't a time entry");
    let time: usize = str::parse(&times_str.split("     ").fold(String::from(""), |mut acc, x| { acc.push_str(x); acc })).expect("boo not a valid usize");
    let distances_str = lines[1].strip_prefix("Distance:   ").expect("boo this isn't a distance entry");
    let distance: usize = str::parse(&distances_str.split("   ").fold(String::from(""), |mut acc, x| { acc.push_str(x); acc })).expect("boo not a valid usize");

    let mut number_of_ways = Vec::new();
    let mut button_times = Vec::new();
    for vel in 1..time {
        // compute distance
        // velocity = vel
        // t = time - vel
        let distance_travelled = vel * (time - vel);
        if distance_travelled > distance {
            button_times.push(vel);
        }
    }
    number_of_ways.push(button_times.len());

    let result: usize = number_of_ways.iter().product();

    println!("result: {}", result);


    Ok(())
} 