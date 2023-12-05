use std::fs::File;
use std::io::Read;
#[allow(unused_assignments)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("./input.txt");
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let chars = string.chars().collect::<Vec<_>>();

    let mut total_sum: Vec<Vec<u32>> = Vec::new();
    let mut line_sum: [u32; 2] = [0, 0];
    let mut count = 0u32;

    chars.iter().for_each(|char| {
        if char.is_numeric() {
            let number = match char.to_digit(10) {
                Some(number) => number,
                None => unreachable!(),
            };
            if count < 1 {
                line_sum[0] = number;
                count += 1;
            } else {
                line_sum[1] = number;
                count += 1;
            }
        }

        if char.is_whitespace() {
            if count == 1 {
                line_sum[1] = line_sum[0];
            }
            total_sum.push(line_sum.to_vec());
            count = 0;
        }
    });

    let mut final_sum = 0;
    total_sum.iter().for_each(|sum| {
        let x = (sum[0] * 10) + sum[1];
        final_sum += x;
        println!("{x}");
    });

    println!("final_sum: {}", final_sum);

    Ok(())
}
