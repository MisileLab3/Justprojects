use clap::Parser;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

/// Simple program to check score.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Answer of questions list (filename(only txt))
    #[clap(long)]
    answers: String,

    /// Input of questions list (filename(only txt))
    #[clap(long)]
    inputs: String,

    /// Is it correct or not correct? (filename(only txt))
    #[clap(long)]
    quesbool: String
}

fn main() {
    let args = Args::parse();
    let (answers, inputs, quesbool) = (args.answers, args.inputs, args.quesbool);

    answerit(answers, inputs, quesbool);
}

fn answerit(answers: String, inputs: String, quesbool: String) {
    let quesanswer = read_lines(format!("{}.txt", answers));
    let quesinput = read_lines(format!("{}.txt", inputs));
    let quesbool = read_lines(format!("{}.txt", quesbool));
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