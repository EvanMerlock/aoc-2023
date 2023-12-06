use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_lapper::{Lapper, Interval};

#[derive(Clone)]
struct IntervalStorage {
    begin_mapping: HashMap<usize, usize>,
    begin_length_mapping: HashMap<usize, usize>,
}

impl IntervalStorage {
    fn find_value(&self, val: usize) -> usize {
        for interval in &self.begin_length_mapping {
            if val >= *interval.0 && val < interval.0 + interval.1 {
                let offset = val - interval.0;
                return self.begin_mapping.get(interval.0).expect("evil") + offset;
            }
        }

        val
    }
}

#[derive(Clone)]
struct NtoMMap {
    n: String,
    m: String,
    intervals: IntervalStorage,
}

impl NtoMMap {
    fn parse(data: &[String]) -> NtoMMap {
        let header = &data[0];
        
        let sp = header.find(' ').expect("bad data format");
        let n_to_m = &header[0..sp];

        let n_and_m = n_to_m.split("-to-").collect::<Vec<&str>>();
        let n = n_and_m[0];
        let m = n_and_m[1];

        let mut n_interval_data = HashMap::new();
        let mut begin_mapping = HashMap::new();

        for line in &data[1..] {
            if line.is_empty() {
                break;
            }

            let numbers = line.split(' ').collect::<Vec<&str>>();
            
            let dest_range_start: usize = str::parse(numbers[0]).expect("failed to parse dest range start");
            let src_range_start: usize = str::parse(numbers[1]).expect("failed to parse dest range start");
            let range_length: usize = str::parse(numbers[2]).expect("failed to parse dest range start");

            n_interval_data.insert(src_range_start, range_length);
            begin_mapping.insert(src_range_start, dest_range_start);
        }

        NtoMMap {
            n: n.to_owned(),
            m: m.to_owned(),
            intervals: IntervalStorage { begin_length_mapping: n_interval_data, begin_mapping },
        }
    }
}

struct NtoMMaps {
    maps: HashMap<String, (NtoMMap, String)>
}

impl NtoMMaps {
    pub fn new(data: &[String]) -> NtoMMaps {
        let mut maps = HashMap::new();

        for section in data.split(|x| x == &String::from("")) {
            let ntom = NtoMMap::parse(section);

            maps.insert(ntom.n.clone(), (ntom.clone(), ntom.m.clone()));
        }

        NtoMMaps { maps }
    }
}

struct Seeds {
    seeds: Vec<usize>
}

impl Seeds {
    fn parse(line: String) -> Seeds {
        let sv = line.split(':').collect::<Vec<&str>>();
        let seeds: Vec<usize> = sv[1].strip_prefix(' ').expect("failed to strip seed header space").split(' ').map(|x| str::parse(x).expect("failed to convert")).collect();
        Seeds { seeds }
    }
}

struct SeedIntervals {
    seeds: Vec<(usize, usize)>
}

impl SeedIntervals {
    fn parse(line: String) -> SeedIntervals {
        let sv = line.split(':').collect::<Vec<&str>>();
        let seeds: Vec<usize> = sv[1].strip_prefix(' ').expect("failed to strip seed header space").split(' ').map(|x| str::parse(x).expect("failed to convert")).collect();

        let si = seeds.chunks(2).map(|x| (x[0], x[1])).collect::<Vec<(usize, usize)>>();

        SeedIntervals { seeds: si }
    }
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines = in_file.lines().map(|x| x.expect("failed to validate line")).collect::<Vec<String>>();

    let seeds = Seeds::parse(lines[0].clone());
    let n_to_m_maps = NtoMMaps::new(&lines[2..]);

    let mut lowest_location = usize::MAX;

    let seed_map = n_to_m_maps.maps.get("seed").expect("no");
    let soil_map = n_to_m_maps.maps.get("soil").expect("no");
    let fertilizer_map = n_to_m_maps.maps.get("fertilizer").expect("no");
    let water_map = n_to_m_maps.maps.get("water").expect("no");
    let light_map = n_to_m_maps.maps.get("light").expect("no");
    let temperature_map = n_to_m_maps.maps.get("temperature").expect("no");
    let humidity_map = n_to_m_maps.maps.get("humidity").expect("no");



    for seed in seeds.seeds {
        println!("===");
        
        let soil_solution = seed_map.0.intervals.find_value(seed);
        let fertilizer_solution = soil_map.0.intervals.find_value(soil_solution);
        let water_solution = fertilizer_map.0.intervals.find_value(fertilizer_solution);
        let light_solution = water_map.0.intervals.find_value(water_solution);
        let temperature_solution = light_map.0.intervals.find_value(light_solution);
        let humidity_solution = temperature_map.0.intervals.find_value(temperature_solution);
        let location = humidity_map.0.intervals.find_value(humidity_solution);

        if location < lowest_location {
            lowest_location = location;
        }
    }

    println!("lowest: {}", lowest_location);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines = in_file.lines().map(|x| x.expect("failed to validate line")).collect::<Vec<String>>();

    let seeds = SeedIntervals::parse(lines[0].clone());
    let n_to_m_maps = NtoMMaps::new(&lines[2..]);

    let seed_map = n_to_m_maps.maps.get("seed").expect("no");
    let soil_map = n_to_m_maps.maps.get("soil").expect("no");
    let fertilizer_map = n_to_m_maps.maps.get("fertilizer").expect("no");
    let water_map = n_to_m_maps.maps.get("water").expect("no");
    let light_map = n_to_m_maps.maps.get("light").expect("no");
    let temperature_map = n_to_m_maps.maps.get("temperature").expect("no");
    let humidity_map = n_to_m_maps.maps.get("humidity").expect("no");

    let v = seeds.seeds.par_iter().flat_map(|x| {
        let mut vv = Vec::new();
        for val in 0..x.1 {
            vv.push(x.0 + val);
        }
        vv
    }).map(|x| {
        let soil_solution = seed_map.0.intervals.find_value(x);
        let fertilizer_solution = soil_map.0.intervals.find_value(soil_solution);
        let water_solution = fertilizer_map.0.intervals.find_value(fertilizer_solution);
        let light_solution = water_map.0.intervals.find_value(water_solution);
        let temperature_solution = light_map.0.intervals.find_value(light_solution);
        let humidity_solution = temperature_map.0.intervals.find_value(temperature_solution);
        humidity_map.0.intervals.find_value(humidity_solution)
    }).reduce(|| usize::MAX, |acc, x| {
        if acc < x {
            acc
        } else {
            x
        }
    });

    println!("lowest: {}", v);

    Ok(())
}