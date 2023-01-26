use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
#[derive(Debug)]
pub enum Result {
    Win(usize),
    Draw(usize),
    Loss(usize),
}
#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    None,
}
// pub struct MyChoice {
//     choice: Choice,
// }
// pub struct TheirChoice {
//     choice: Choice,
// }
#[derive(Debug)]
pub struct Day2 {
    // my_choice: Choice,
    // their_choice: Choice,
    choices: Vec<(Choice, Choice)>,
}

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        // parse input from Strings and convert to choices

        // lines.lines().map(|l| l.split(' '));
        // println!("{our_choice:#?}");
        let mut choices = Vec::new();
        for line in lines {
            let choice = line;
            // choice.split(' ').collect::<Vec<&str>>();
            let mut us = Choice::None;
            let mut them = Choice::None;
            for chaar in choice.split(' ') {
                match chaar {
                    "A" => {
                        them = Choice::Rock;
                        // them_vec.push(them);
                    }
                    "X" => {
                        us = Choice::Rock;
                        // us_vec.push(us);
                    }
                    "B" => {
                        them = Choice::Paper;
                        // them_vec.push(them);
                    }
                    "Y" => {
                        us = Choice::Paper;
                        // us_vec.push(us);
                    }
                    "C" => {
                        them = Choice::Scissors;
                        // them_vec.push(them);
                    }
                    "Z" => {
                        us = Choice::Scissors;
                        // us_vec.push(us);
                    }
                    _ => todo!("Input not expected!"),
                };
            }
            let result = (us, them);
            choices.push(result);
        }
        // println!("{our_choice:#?}");
        // return our Day2 struct with the Vecs:
        Day2 { choices }
        // todo!("Parse your input from the input file")
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        // take our array/itor of Choices
        // we just need to do the math here
        // left is theirs and right is ours
        let choices = &self.choices;

        let mut score = 0;
        let res = choices.iter();
        for vs in res {
            match vs {
                (Choice::Rock, Choice::Rock) => {
                    score += 1;
                    score += 3;
                }
                (Choice::Rock, Choice::Paper) => {
                    score += 1;
                    score += 0;
                }
                (Choice::Rock, Choice::Scissors) => {
                    score += 1;
                    score += 6;
                }
                (Choice::Paper, Choice::Rock) => {
                    score += 2;
                    score += 6;
                }
                (Choice::Paper, Choice::Paper) => {
                    score += 2;
                    score += 3;
                }
                (Choice::Paper, Choice::Scissors) => {
                    score += 2;
                    score += 0;
                }
                (Choice::Scissors, Choice::Rock) => {
                    score += 3;
                    score += 0;
                }
                (Choice::Scissors, Choice::Paper) => {
                    score += 3;
                    score += 6;
                }
                (Choice::Scissors, Choice::Scissors) => {
                    score += 3;
                    score += 3;
                }
                (_, _) => todo!("This can't happen... Rightt.."),
            }
        }

        // let result = match our_input {
        //     Day2::Rock => todo!(),
        //     Day2::Paper=> todo!(),
        //     Day2::Scissors=> todo!(),

        // }
        // todo!("Solve part one of day 2 using your parsed input")
        println!("{self:?}");
        score.to_string()
    }

    fn part_two(&self) -> String {
        // todo!("Solve part two of day 2 using your parsed input")
        "".to_string()
    }
}
