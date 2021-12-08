use std::env;

mod days;
mod helper;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let input_data = include_input_data!("day1", "day2", "day3", "day4", "day6", "day7");

    if let Some(arg) = args.get(1) {

        let day_to_run = match arg.as_str() {
            "day1" | "1" => days::day1::run,
            "day2" | "2" => days::day2::run,
            "day3" | "3" => days::day3::run,
            "day4" | "4" => days::day4::run,
            "day6" | "6" => days::day6::run,
            "day7" | "7" => days::day7::run,
            _ => do_nothing
        };

        day_to_run(&input_data)?;
    } else {
        // Run all days
        days::day1::run(&input_data)?;
        days::day2::run(&input_data)?;
        days::day3::run(&input_data)?;
        days::day4::run(&input_data)?;
        days::day6::run(&input_data)?;
        days::day7::run(&input_data)?;
    }


    Ok(())
}

fn do_nothing(_x: &[(&str, &str)]) -> anyhow::Result<()> {
    println!("not a valid argument");
    Ok(())
}