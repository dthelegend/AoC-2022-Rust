use std::io;

fn main() {
    let mut score: i32 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(line_str) => {
                let (enemy_move, outcome) = match line_str.split(' ').collect::<Vec<&str>>()[..] {
                    [enemy_move, outcome] => (enemy_move, outcome),
                    _ => panic!("Invalid input line {}", line_str)
                };
                score += match enemy_move {
                    // Rock
                    "A" => match outcome {
                        // Loss
                        "X" => 3,
                        // Draw
                        "Y" => 1,
                        // Win
                        "Z" => 2,
                        // Something Else
                        _ => panic!("Problem reading line, failed to parse outcome \"{}\"", enemy_move)
                    },
                    // Paper
                    "B" => match outcome {
                        // Loss
                        "X" => 1,
                        // Draw
                        "Y" => 2,
                        // Win
                        "Z" => 3,
                        // Something Else
                        _ => panic!("Problem reading line, failed to parse outcome \"{}\"", enemy_move)
                    },
                    // Scissors
                    "C" => match outcome {
                        // Loss
                        "X" => 2,
                        // Draw
                        "Y" => 3,
                        // Win
                        "Z" => 1,
                        // Something Else
                        _ => panic!("Problem reading line, failed to parse outcome \"{}\"", enemy_move)
                    },
                    // Something Else
                    _ => panic!("Problem reading line, failed to parse move \"{}\"", enemy_move)
                };
                score += match outcome {
                    // Loss
                    "X" => 0,
                    // Draw
                    "Y" => 3,
                    // Win
                    "Z" => 6,
                    // Something Else
                    _ => panic!("Problem reading line, failed to parse outcome \"{}\"", enemy_move)
                };
            },
            Err(err) => panic!("Problem reading line\n{}", err)
        }
    }
    println!("{}", score);
}