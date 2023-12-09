use std::{io::{BufReader, BufRead}, fs::File};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(PartialEq, Clone)]
struct Sequence {
    inner: Vec<isize>,
}

impl Sequence {
    fn parse(line: String) -> Sequence {
        let seq: Vec<isize> = line.split(' ').map(|x| str::parse(x).expect("boo not an isize")).collect();

        Sequence { inner: seq }
    }

    fn next_sequence(&self) -> Sequence {
        let mut seq = Vec::new();
        for (idx, item) in self.inner.iter().enumerate() {
            if idx != self.inner.len() - 1 {
                seq.push(self.inner[idx+1] - item);
            }
        }

        Sequence { inner: seq }
    }

    fn is_last_seq(&self) -> bool {
        self.inner.iter().all(|x| *x == 0)
    }

    fn predict(&self, next: isize) -> isize {
        self.inner.last().expect("no element") + next
    }

    fn predict_prev(&self, prev: isize) -> isize {
        self.inner.first().expect("no element") - prev
    }
}

struct SequenceCollection {
    starting_collection: Sequence,
    collection: Vec<Sequence>
}

impl SequenceCollection {
    fn parse(line: String) -> SequenceCollection {
        let seq = Sequence::parse(line);
        SequenceCollection { starting_collection: seq.clone(), collection: vec![seq] }
    }

    fn fill(&mut self) {
        let mut starting_seq = self.starting_collection.clone();

        while !starting_seq.is_last_seq() {
            starting_seq = starting_seq.next_sequence();
            self.collection.push(starting_seq.clone());
        }
    }

    fn predict(&self) -> isize {
        let mut seq_index = self.collection.len() - 1;
        let mut seq = self.collection[seq_index].clone();
        let mut diff = 0;
        // skips out on running prediction on starting collection
        while seq != self.starting_collection {
            diff = seq.predict(diff);
            seq_index -= 1;
            seq = self.collection[seq_index].clone();
        }

        self.starting_collection.predict(diff)
    }

    fn predict_prev(&self) -> isize {
        let mut seq_index = self.collection.len() - 1;
        let mut seq = self.collection[seq_index].clone();
        let mut diff = 0;
        // skips out on running prediction on starting collection
        while seq != self.starting_collection {
            diff = seq.predict_prev(diff);
            seq_index -= 1;
            seq = self.collection[seq_index].clone();
        }

        self.starting_collection.predict_prev(diff)
    }
}


pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo! bad string")).collect();
    let mut collections: Vec<SequenceCollection> = lines.into_iter().map(SequenceCollection::parse).collect();
    collections = collections.into_iter().map(|mut x| { x.fill(); x }).collect();
    let result: isize = collections.par_iter().map(|x| x.predict()).sum();

    println!("result: {}", result);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo! bad string")).collect();
    let mut collections: Vec<SequenceCollection> = lines.into_iter().map(SequenceCollection::parse).collect();
    collections = collections.into_iter().map(|mut x| { x.fill(); x }).collect();
    let result: isize = collections.par_iter().map(|x| x.predict_prev()).sum();

    println!("result: {}", result);

    Ok(())
}