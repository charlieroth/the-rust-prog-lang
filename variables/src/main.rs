const HOURS: u32 = 3;
const SECONDS_IN_HOUR: u32 = 60 * 60;

fn main() {
    let seconds_in_three_hours = SECONDS_IN_HOUR * HOURS;
    println!("There are {seconds_in_three_hours} seconds in {HOURS} hours");
}
