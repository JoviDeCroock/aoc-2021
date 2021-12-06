use std::fs::File;
use std::io::Read;

fn main() {
    let mut content = String::new();
    let mut f = File::open("./src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut content).expect("Unable to read string");

    let lines = content.lines();

    let mut coordinates = (0, 0);
    for line in lines {
        let vec: Vec<&str> = line.split(" ").collect();
        let word = vec[0];
        let count = vec[1].parse::<i32>().unwrap();
        match word {
            "forward" => coordinates.1 = coordinates.1 + count,
            "down" => coordinates.0 = coordinates.0 + count,
            "up" => coordinates.0 = coordinates.0 - count,
            _ => println!("Ain't special"),
        }
    }

    println!("{}", coordinates.0 * coordinates.1);

    let lines = content.lines();

    let mut coordinates = (0, 0, 0);
    for line in lines {
        let vec: Vec<&str> = line.split(" ").collect();
        let word = vec[0];
        let count = vec[1].parse::<i32>().unwrap();
        match word {
            "forward" => {
                coordinates.1 = coordinates.1 + count;
                coordinates.0 = coordinates.0 + (coordinates.2 * count);
            },
            "down" => coordinates.2 = coordinates.2 + count,
            "up" => coordinates.2 = coordinates.2 - count,
            _ => println!("Ain't special"),
        }
    }

    println!("{}", coordinates.0 * coordinates.1);
}
