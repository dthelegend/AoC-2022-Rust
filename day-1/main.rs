use std::io;

fn main() {
    let mut elf_calories: Vec<i64> = Vec::with_capacity(3);
    let lines = io::stdin().lines();

    let mut cur_elf_calories: i64 = 0;
    for line in lines {
        match line {
            Ok(line_str) => {
                if line_str.is_empty() {
                    let mut insert_index = 0;
                    for top_elf_cal in &elf_calories {
                        if cur_elf_calories > *top_elf_cal  {
                            break;
                        }
                        insert_index += 1;
                    }
                    elf_calories.insert(insert_index, cur_elf_calories);
                    while elf_calories.len() > 3 {
                        elf_calories.pop();
                    }
                    cur_elf_calories = 0;
                    continue;
                }
                cur_elf_calories += match line_str.parse::<i64>() {
                    Ok(n) => n,
                    Err(err) => panic!("Problem reading line, failed to parse input \"{}\"\n{}", line_str, err)
                };
            },
            Err(err) => panic!("Problem reading line\n{}", err)
        };
    }
    println!("{:?}", elf_calories);
    println!("{}", elf_calories.iter().sum::<i64>());
}