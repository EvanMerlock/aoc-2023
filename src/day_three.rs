use std::collections::HashMap;
use std::ops::Index;
use std::{io::BufReader, fs::File};
use std::io::BufRead;

struct EngineSchematic {
    rows: Vec<Row>
}

struct Row {
    inner: Vec<char>,
}

struct UnverifiedPartNumber {
    num: i64,
    locations: Vec<Coordinate>
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PartNumber {
    num: i64,
    locations: Vec<Coordinate>
}

struct UnverifiedGear {
    location: Coordinate
}

struct Gear {
    gear_ratio: i64,
    locations: Coordinate
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    row: usize,
    column: usize
}

impl Coordinate {
    fn get_neighboring(&self) -> Vec<Coordinate> {
        let mut neighboring = Vec::new();

        match (self.row, self.column) {
            (0, 0) => {
                neighboring.push(Coordinate::from((1,0)));
                neighboring.push(Coordinate::from((0,1)));
                neighboring.push(Coordinate::from((1,1)));
            },
            (row, 0) => {
                neighboring.push(Coordinate::from((row+1,1)));
                neighboring.push(Coordinate::from((row+1,0)));

                neighboring.push(Coordinate::from((row-1,1)));
                neighboring.push(Coordinate::from((row-1,0)));

                neighboring.push(Coordinate::from((row,1)));
            },
            (0, column) => {
                neighboring.push(Coordinate::from((0, column-1)));
                neighboring.push(Coordinate::from((0, column+1)));

                neighboring.push(Coordinate::from((1,column-1)));
                neighboring.push(Coordinate::from((1,column+1)));

                neighboring.push(Coordinate::from((1,column)));
            },
            (row, column) => {
                neighboring.push(Coordinate::from((row+1, column+1)));
                neighboring.push(Coordinate::from((row-1, column-1)));

                neighboring.push(Coordinate::from((row-1, column+1)));
                neighboring.push(Coordinate::from((row+1, column-1)));

                neighboring.push(Coordinate::from((row+1, column)));
                neighboring.push(Coordinate::from((row-1, column)));

                neighboring.push(Coordinate::from((row, column+1)));
                neighboring.push(Coordinate::from((row, column-1)));
            }
        }

        neighboring
    }

    fn exceeds_bounds(&self, row_len: usize, col_len: usize) -> bool {
        self.row >= row_len || self.column >= col_len
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Coordinate { row: value.0, column: value.1 }
    }
}


impl Row {
    fn from(row: String) -> Row {
        Row {
            inner: row.chars().collect::<Vec<char>>()
        }
    }

    fn get_unverified_part_numbers(&self, row_number: usize) -> Vec<UnverifiedPartNumber> {
        let mut upns = Vec::new();

        let mut buf = String::new();
        let mut offsets = Vec::new();
        for (loc, char) in self.inner.iter().enumerate() {
            if !char.is_ascii_digit() {
                if !buf.is_empty() {
                    let num = str::parse::<i64>(&buf).expect("failed to parse string");
                    upns.push(UnverifiedPartNumber { num, locations: offsets.iter().map(|x| Coordinate::from((row_number, *x))).collect() });
                    buf = String::new();
                    offsets = Vec::new();
                }


                continue;
            }

            buf.push(*char);
            offsets.push(loc);
        };

        if !buf.is_empty() {
            let num = str::parse::<i64>(&buf).expect("failed to parse string");
            upns.push(UnverifiedPartNumber { num, locations: offsets.iter().map(|x| Coordinate::from((row_number, *x))).collect() });
        }

        upns
    }

    fn get_unverified_gears(&self, row_number: usize) -> Vec<UnverifiedGear> {
        let mut ugs = Vec::new();

        for (loc, char) in self.inner.iter().enumerate() {
            if *char == '*' {
                ugs.push(UnverifiedGear { location: Coordinate::from((row_number, loc)) });
            }
        }

        ugs
    }
}

impl Index<usize> for Row {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl EngineSchematic {
    fn from(file: BufReader<File>) -> Result<EngineSchematic, std::io::Error> {
        let mut rows = Vec::new();
        for line in file.lines() {
            let validated_line = line?;

            rows.push(Row::from(validated_line));
        }

        Ok(EngineSchematic { rows })
    }

    fn get_verified_part_numbers(&self) -> Vec<PartNumber> {
        let mut full_upns = Vec::new();
        let mut pns = Vec::new();

        for (offset, row) in self.rows.iter().enumerate() {
            let mut upns = row.get_unverified_part_numbers(offset);
            full_upns.append(&mut upns);
        }

        for upn in full_upns {
            for coord in &upn.locations {
                if self.coordinate_adjacent_symbol(*coord) {
                    pns.push(PartNumber { num: upn.num, locations: upn.locations.clone() });
                    break;
                }      
            }
        }

        pns
    }

    fn coordinate_adjacent_symbol(&self, coord: Coordinate) -> bool {
        for adj in coord.get_neighboring() {
            if adj.exceeds_bounds(self.rows.len(), self.rows[0].inner.len()) {
                continue;
            }
            if is_symbol(self[adj]) {
                return true;
            }
        }

        false
    }

    fn get_unverified_gears(&self) -> Vec<UnverifiedGear> {
        let mut full_gears = Vec::new();

        for (offset, row) in self.rows.iter().enumerate() {
            let mut upns = row.get_unverified_gears(offset);
            full_gears.append(&mut upns);
        }

        full_gears
    }

    fn get_verified_gears(&self, map: HashMap<Coordinate, &PartNumber>) -> Vec<Gear> {
        let mut full_gears = Vec::new();
        let mut verified_gears = Vec::new();

        for (offset, row) in self.rows.iter().enumerate() {
            let mut upns = row.get_unverified_gears(offset);
            full_gears.append(&mut upns);
        }

        for gear in full_gears {
            let mut adjacent_parts: HashMap<&PartNumber, ()> = HashMap::new();
            let adj = gear.location.get_neighboring();
            for coord in adj {
                if let Some(pn) = map.get(&coord) {
                    adjacent_parts.insert(pn, ());
                }
            }

            if adjacent_parts.len() == 2 {
                let gr = adjacent_parts.keys().fold(1, |acc, k| acc * k.num);
                verified_gears.push(Gear { gear_ratio: gr, locations: gear.location });
            }
        }

        verified_gears
    }
}

fn is_symbol(item: char) -> bool {
    !item.is_ascii_digit() && item != '.'
}

impl Index<Coordinate> for EngineSchematic {
    type Output = char;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.rows[index.row][index.column]
    }
}

fn convert_pns_to_map(pns: &[PartNumber]) -> HashMap<Coordinate, &PartNumber> {
    let mut res = HashMap::new();

    for pn in pns {
        for coord in &pn.locations {
            res.insert(*coord, pn);
        }
    }

    res
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let engine_schematic = EngineSchematic::from(in_file)?;

    let mut total = 0;
    for pn in engine_schematic.get_verified_part_numbers() {
        total += pn.num;
    }
    
    println!("total: {}", total);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let engine_schematic = EngineSchematic::from(in_file)?;

    let pns = engine_schematic.get_verified_part_numbers();
    let mapping = convert_pns_to_map(&pns);
    let mut total = 0;
    for pn in engine_schematic.get_verified_gears(mapping) {
        total += pn.gear_ratio;
    }
    
    println!("total: {}", total);

    Ok(())
}