use std::collections::BTreeMap;

pub fn day1(part_two: bool) {
    let ls = std::io::stdin().lines().map(|l| l.unwrap());
    let res = if part_two {
        day1_part2(ls)
    } else {
        day1_part1(ls)
    };
    println!("{}", res);
}

fn parse_input(ls: impl Iterator<Item = String>) -> Vec<(i32, i32)> {
    ls.map(|l| {
        l.split_whitespace()
            .map(|s| s.parse().expect("Could not parse input number"))
            .collect::<Vec<i32>>()
    })
    .map(|ns| match ns.as_slice() {
        [x, y] => (*x, *y),
        _ => panic!("Wrong number of numbers on line"),
    })
    .collect()
}

fn day1_part1(ls: impl Iterator<Item = String>) -> i32 {
    let pairs = parse_input(ls);
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

fn day1_part2(ls: impl Iterator<Item = String>) -> i32 {
    let pairs = parse_input(ls);
    let lefts: Vec<i32> = pairs.iter().map(|(x, _)| *x).collect();
    let rights: BTreeMap<i32, i32> =
        pairs
            .iter()
            .map(|(_, y)| *y)
            .fold(BTreeMap::new(), |mut p, y| {
                p.entry(y).and_modify(|c| *c = *c + 1).or_insert(1);
                p
            });

    lefts
        .into_iter()
        .map(|x| rights.get(&x).unwrap_or(&0) * x)
        .sum()
}

#[cfg(test)]
mod test {
    fn test_input() -> impl Iterator<Item = String> {
        "3   4
4   3
2   5
1   3
3   9
3   3"
            .lines()
            .map(|s| s.to_string())
    }

    #[test]
    fn test_day1_part1() {
        assert_eq!(super::day1_part1(test_input()), 11);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(super::day1_part2(test_input()), 31);
    }
}
