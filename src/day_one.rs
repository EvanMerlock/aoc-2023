use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};
use regex::Regex;
use regex::RegexSet;

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {

    let long_regex = Regex::new(r"(\d).*(\d)").unwrap();
    let short_regex = Regex::new(r".*(\d)").unwrap();

    let regex_set = RegexSet::new([
        r"(\d).*(\d)",
        r".*(\d)"
    ]).unwrap();
    let mut total = 0;

    for line in in_file.lines() {
        let verified_line = line?;
        let matches = regex_set.matches(&verified_line);
        if matches.matched(0) {
            for (_, [first, last]) in long_regex.captures_iter(&verified_line).map(|c| c.extract()) {
                let fmt_str = format!("{}{}", first, last);
                let sum = str::parse::<i32>(&fmt_str).unwrap();
                total += sum;
            }
        } else if matches.matched(1) {
            for (_, [first]) in short_regex.captures_iter(&verified_line).map(|c| c.extract()) {
                let fmt_str = format!("{}{}", first, first);
                let sum = str::parse::<i32>(&fmt_str).unwrap();
                total += sum;
            }
        }
    }
    println!("total is: {}", total);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let long_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d).*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let short_regex = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let regex_set = RegexSet::new([
        r"(one|two|three|four|five|six|seven|eight|nine|\d).*(one|two|three|four|five|six|seven|eight|nine|\d)",
        r".*(one|two|three|four|five|six|seven|eight|nine|\d)"
    ]).unwrap();

    let mut mapping_hm = HashMap::new();
    mapping_hm.insert("one", "1");
    mapping_hm.insert("two", "2");
    mapping_hm.insert("three", "3");
    mapping_hm.insert("four", "4");
    mapping_hm.insert("five", "5");
    mapping_hm.insert("six", "6");
    mapping_hm.insert("seven", "7");
    mapping_hm.insert("eight", "8");
    mapping_hm.insert("nine", "9");


    let mut total = 0;

    for line in in_file.lines() {
        let verified_line = line?;
        let matches = regex_set.matches(&verified_line);
        if matches.matched(0) {
            for (_, [first, last]) in long_regex.captures_iter(&verified_line).map(|c| c.extract()) {
                let fixed_first = if let Some(mapped_first) = mapping_hm.get(first) {
                    mapped_first
                } else {
                    first
                };
                let fixed_last = if let Some(mapped_last) = mapping_hm.get(last) {
                    mapped_last
                } else {
                    last
                };
                let fmt_str = format!("{}{}", fixed_first, fixed_last);
                println!("{}", fmt_str);
                let sum = str::parse::<i32>(&fmt_str).unwrap();
                total += sum;
            }
        } else if matches.matched(1) {
            for (_, [first]) in short_regex.captures_iter(&verified_line).map(|c| c.extract()) {
                let fixed_first = if let Some(mapped_first) = mapping_hm.get(first) {
                    mapped_first
                } else {
                    first
                };
                let fmt_str = format!("{}{}", fixed_first, fixed_first);
                let sum = str::parse::<i32>(&fmt_str).unwrap();
                total += sum;
            }
        }
    }
    println!("total is: {}", total);


    Ok(())
}