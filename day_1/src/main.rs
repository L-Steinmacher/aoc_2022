use std::{fs::File, env, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).expect("There was an error opening the file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("There was an error reading line");

    let mut calorie_count_totals = Vec::<i32>::new();
    let mut calories_so_far= 0;

    for line in file_content.lines().into_iter() {
        if line.is_empty() {
            calorie_count_totals.push(calories_so_far);
            calories_so_far = 0;
            continue;
        }
        calories_so_far = calories_so_far + line.parse::<i32>().unwrap();
    }

    calorie_count_totals.push(calories_so_far);
    calorie_count_totals.sort();
    print!("{}", calorie_count_totals.clone().last().unwrap());

}
