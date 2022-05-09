use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use clap::Parser;

/// Simple program for check this is false test point.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Arguments {
    /// Question answer (only txt, line break)
    #[clap(long)]
    quesanswer: String,

    /// Answer that you input to question (only txt, line break)
    #[clap(long)]
    quesinput: String,

    /// That question input and answer is same? (only txt, line break)
    #[clap(long)]
    quesbool: String
}


fn main() {
    answerit();
}

fn answerit() {
    let args = Arguments::parse();

    let quesanswer = read_lines(args.quesanswer);
    let quesinput = read_lines(args.quesinput);
    let quesbool = read_lines(args.quesbool);
    let mut falselist: Vec<usize> = Vec::new();

    for i in 0..quesinput.len() {
        println!("{}: {} - {}", quesinput[i], quesanswer[i], quesbool[i]);
        if quesbool[i].parse::<bool>().unwrap() != (quesanswer[i] == quesinput[i]) {
            falselist.push(i + 1);
        }
    }

    if falselist.len() == 0 {
        println!("No wrong question!")
    } else {
        println!("Wrong question (or mistake input): {:#?}", falselist)
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let mut vector = vec![];
    for i in BufReader::new(file).lines() {
        vector.push(i.unwrap())
    };
    vector
}