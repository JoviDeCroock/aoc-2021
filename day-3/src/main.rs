use std::fs::File;
use std::io::Read;

fn main() {
    let mut content = String::new();
    let mut f = File::open("./src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut content).expect("Unable to read string");

    let gamma = content
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; 12], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= 1000 / 2) as u32) << i)
        .sum::<u32>();

    println!("{}", gamma * (!gamma & ((1 << 12) - 1)));

    // TODO: ex2 https://adventofcode.com/2021/day/3#part2
}
