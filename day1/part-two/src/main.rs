use std::fs::File;
use std::io::{Read, Write};

fn str_to_digit(num: &str) -> usize {
    match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./input.txt")?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let mut strings = string.split('\n').collect::<Vec<_>>();
    strings.pop();
    let number_as_str: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut sum = 0;
    let mut result: Vec<usize> = vec![];

    for string in strings {
        let mut contain_results: Vec<&str> = Vec::new();
        for number in number_as_str {
            if string.matches(number).collect::<Vec<_>>().len() == 1 {
                contain_results.push(number);
            } else if string.matches(number).collect::<Vec<_>>().len() > 1 {
                contain_results.push(number);
                contain_results.push(number);
            }
        }

        let mut index_l: Vec<(usize, &str)> = vec![];
        let mut current_number = "";

        for value in contain_results {
            if current_number == value {
                let ptr = string.as_ptr();
                for i in 0..=string.len() {
                    if value.len() > 1 {
                        unsafe {
                            if *ptr.add(i) == (value.chars().nth(0).unwrap() as u8)
                                && *ptr.add(i + 1) == (value.chars().nth(1).unwrap() as u8)
                            {
                                index_l.push((i, value));
                            }
                        }
                    }
                }
            }
            let index = string.find(value);
            if index.is_none() {
                continue;
            }
            index_l.push((index.unwrap(), value));
            current_number = value;
        }

        if index_l.len() == 1 {
            let first_num = str_to_digit(index_l.first().unwrap().1);
            sum += first_num;
            sum *= 11;
            result.push(sum);
            println!("{sum}");
            sum = 0;
            continue;
        }

        index_l.sort_by(|a, b| a.0.cmp(&b.0));

        let first_num = str_to_digit(index_l.first().unwrap().1);
        let second_num = str_to_digit(index_l.last().unwrap().1);
        sum += first_num;
        sum *= 10;
        sum += second_num;
        println!("{sum}");
        result.push(sum);
        sum = 0;
    }

    let mut final_sum = 0;
    for num in result {
        final_sum += num;
    }
    println!("final_sum: {final_sum}");
    Ok(())
}
