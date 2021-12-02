use std::env;

mod days;
mod helper;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let input_data = include_input_data!("day1", "day2");

    if let Some(arg) = args.get(1) {

        let day_to_run = match arg.as_str() {
            "day1" | "1" => days::day1::run,
            "day2" | "2" => days::day2::run,
            _ => do_nothing
        };

        day_to_run(&input_data);
    } else {
        // Run all days
        days::day1::run(&input_data);
        days::day2::run(&input_data);
    }


    Ok(())
}

fn do_nothing(_x: &[(&str, &str)]) {
    println!("not a valid argument");
}