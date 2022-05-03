use std::fs::File;
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;

fn main() {
    answerit();
}

fn answerit() {
    let pointinput = read_lines("pointinput.txt").unwrap();
    let quespoint = read_lines("quespoint.txt").unwrap();
    let quesanswer = read_lines("quesanswer.txt").unwrap();
    let quesinput = read_lines("quesinput.txt").unwrap();
    let point: u8 = 0;

    for (i, i2, i3) in (quespoint, quesanswer, quesinput) {
        if i2 == i3 {
            point = point + i
        } else {
            point = point - i
        }
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}