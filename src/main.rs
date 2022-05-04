use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    answerit();
}

fn answerit() {
    let status = self_update::backends::github::Update::configure()
    .repo_owner("jaemk")
    .repo_name("self_update")
    .bin_name("github")
    .show_download_progress(true)
    .current_version(cargo_crate_version!())
    .build().unwrap()
    .update().unwrap();
    println!("Update status: `{}`!", status.version());

    let quesanswer = read_lines("quesanswer.txt");
    let quesinput = read_lines("quesinput.txt");
    let quesbool = read_lines("quesbool.txt");
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