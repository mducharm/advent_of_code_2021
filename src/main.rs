mod days;
mod helper;

fn main() -> anyhow::Result<()> {
    let input_data = include_input_data!("day1");

    days::day1::run(&input_data);


    Ok(())
}
