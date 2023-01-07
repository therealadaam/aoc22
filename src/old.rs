use std::fs;
use std::io;

const INPUT_PATH: &str = "./input/";

#[derive(Debug)]
struct Day {
    num: usize,
    data: String,
}

impl Day {
    fn debug() {}
}

fn main() {
    // Get the day first
    let day = get_day();
    // We will start by parsing the input from a text filee
    // parse_input(day);
    // // next we'll do the day specific items to parse
    let data = parse_day_input(day);
    // println!("{:#?}",data);

    // after the day specific we will solve the part 1
    // let max = solve_1(data);
    // println!("{max:?}");
    // now we solve for part 2
    let max = solve_2(data);
    println!("{max:?}");
}
// give this a day in string, conver to usize and return Day Struct
fn get_day() -> Day {
    println!("What Day?");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    // println!("{:#?}",num);
    // let num = read_to_string(io::stdin()).unwrap().parse::<usize>().unwrap();
    // num should be a usize or u32 now.
    // let num = num.parse::<usize>().unwrap();
    let num: usize = num.trim().parse().unwrap();
    let data = parse_input(num);
    Day { num, data }
}

fn parse_day_input(day: Day) -> Vec<Option<usize>> {
    let lines: Vec<_> = day
        .data
        // now we split our data by a blank line
        // I think this is the same as .lines()....
        // .split("\n")
        .lines()
        .map(|line| line.trim().parse::<usize>().ok())
        .collect();
    lines
}

fn parse_input(day: usize) -> String {
    let day_string = day.to_string();
    let path = format!("{INPUT_PATH}{day_string}.txt");
    let data = fs::read_to_string(path).unwrap();
    data
}
fn solve_1(data: Vec<Option<usize>>) -> usize {
    /*
    this returns a sum from the largest group from our list of values
    these values are deliminated by a 0
    let largest: Vec<&[usize]> = data
                    .split(|num| num != &0)
                    .sum::<S>()
                    // .map(|&line| &line.sum())
                    .collect();
    */

    let group = data
        .split(|num| num.is_none())
        .map(|g| g.iter().map(|v| v.unwrap()).sum::<usize>())
        // .collect::<Vec<_>>();
        .max();
    // println!("{group:?}");
    group.unwrap()
}
fn solve_2(data: Vec<Option<usize>>) -> usize {
    let mut group = data
        .split(|num| num.is_none())
        .map(|g| g.iter().map(|v| v.unwrap()).sum::<usize>())
        .collect::<Vec<_>>();
    group.sort();
    let group = group.split_off(&group.len() - 3);
    let data = group.iter().sum();
    println!("{data:?}");
    data
}
