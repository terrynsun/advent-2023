#![allow(dead_code)]

use clap::Parser;

use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

/// T is the type that the input gets parsed into
/// R is the type that the answer comes in
pub struct Puzzle<T, R> {
    pub name: &'static str,
    pub parts: Vec<fn(&T) -> R>,
    pub delimiter: char,
    pub preprocess: fn(Vec<String>) -> T,
}

impl<T, R> Puzzle<T, R> {
    pub fn new(
        name: &'static str,
        parts: Vec<fn(&T) -> R>,
        delimiter: char,
        preprocess: fn(Vec<String>) -> T,
    ) -> Self {
        Puzzle {
            name,
            parts,
            delimiter,
            preprocess,
        }
    }

    pub fn solve(self)
    where
        R: std::fmt::Display,
    {
        let dir = "inputs";

        let args = Args::parse();
        let debug = args.debug;

        let filename = if debug {
            "test.txt".to_string()
        } else {
            format!("{}/{}", dir, self.name)
        };

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines = contents
            .trim_end()
            .split(self.delimiter)
            .map(|x| x.to_string())
            .collect();

        let data = (self.preprocess)(lines);

        for f in self.parts {
            println!("=> {}", f(&data));
        }
    }
}
