mod day1;
mod day2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).expect("No arg provided on the cmd line");
    let part_two = match args.get(2).map(|s| s.as_str()) {
        Some("1") => false,
        Some("2") => true,
        Option::None => false,
        Some(s) => panic!("Unexpected part '{}' provided", s),
    };
    match day.as_str() {
        "1" => self::day1::day1(part_two),
        "2" => self::day2::day2(part_two),
        d => panic!("Day '{}' has not been written yet", d),
    }
}
