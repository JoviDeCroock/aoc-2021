use std::fs::File;
use std::io::Read;

fn main() {
    let mut content = String::new();
    let mut f = File::open("./src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut content).expect("Unable to read string");

    let mut lines = content.lines();
    let mut increased = 0;
    let mut last_number = lines.next().unwrap_or("0").parse::<i32>().unwrap();
    for line in lines {
        let number = line.parse::<i32>().unwrap();
        if number > last_number {
            increased = increased + 1;
        }
        last_number = number;
    }

    println!("Ex 1: Increased {} times", increased);

    let lines = content.lines();
    let count = lines.count() - 2;
    let mut i = 0;

    let lines: Vec<i32> = content.lines().map(|value| value.parse().unwrap_or(0)).collect();
    let mut increased = 0;
    let mut last_number = 0;

    while i < count {
        let el1 = lines[i];
        let el2 = lines[i + 1];
        let el3 = lines[i + 2];
        let number = el1 + el2 + el3;

        if i != 0 && number > last_number {
            increased = increased + 1;
        }

        last_number = number;
        i = i + 1;
    }

    println!("Ex 2: Increased {} times", increased);
}
