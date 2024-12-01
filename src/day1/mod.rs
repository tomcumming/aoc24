pub fn day1(part_two: bool) {
    assert!(!part_two);

    let ls = std::io::stdin().lines().map(|l| l.unwrap());
    println!("{}", day1_part1(ls));
}

fn day1_part1(ls: impl Iterator<Item = String>) -> i32 {
    let pairs: Vec<(i32, i32)> = ls
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().expect("Could not parse input number"))
                .collect::<Vec<i32>>()
        })
        .map(|ns| match ns.as_slice() {
            [x, y] => (*x, *y),
            _ => panic!("Wrong number of numbers on line"),
        })
        .collect();
    let mut lefts: Vec<i32> = pairs.iter().map(|(x, _)| *x).collect();
    lefts.sort();
    let mut rights: Vec<i32> = pairs.iter().map(|(_, y)| *y).collect();
    rights.sort();

    lefts
        .into_iter()
        .zip(rights.into_iter())
        .map(|(x, y)| i32::abs(x - y))
        .sum()
}

#[test]
fn test_day1_part1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3"
        .lines()
        .map(|s| s.to_string());

    assert_eq!(day1_part1(input), 11);
}
