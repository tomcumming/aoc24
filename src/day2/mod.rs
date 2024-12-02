pub fn day2(part_two: bool) {
    let ls = std::io::stdin().lines().map(|l| l.unwrap());
    let res = if part_two {
        day2_part2(ls)
    } else {
        day2_part1(ls)
    };
    println!("{}", res);
}

fn parse_input(ls: impl Iterator<Item = String>) -> Vec<Vec<i32>> {
    ls.map(|l| {
        l.split_whitespace()
            .map(|s| s.parse().expect("Could not parse input number"))
            .collect::<Vec<i32>>()
    })
    .collect()
}

fn safe_pair(dir: i32, a: i32, b: i32) -> bool {
    let delta = b - a;
    let m = i32::abs(delta);
    i32::signum(delta) == dir && m <= 3 && m >= 1
}

fn no_skips(dir: i32, mut ns: &[i32]) -> bool {
    loop {
        match ns {
            [a, b, ..] if safe_pair(dir, *a, *b) => {
                ns = &ns[1..];
            }
            _ => return ns.len() <= 1,
        }
    }
}

fn one_skip(dir: i32, ns: &[i32]) -> bool {
    if no_skips(dir, ns) {
        return true;
    }

    // I said I would be lazy this year...
    (0..ns.len())
        .map(|i| {
            ns.iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, n)| n)
                .cloned()
                .collect()
        })
        .any(|xs: Vec<i32>| no_skips(dir, &xs))
}

fn day2_part1(ls: impl Iterator<Item = String>) -> usize {
    parse_input(ls)
        .into_iter()
        .filter(|ns| no_skips(1, ns.as_slice()) || no_skips(-1, ns.as_slice()))
        .count()
}

fn day2_part2(ls: impl Iterator<Item = String>) -> usize {
    parse_input(ls)
        .into_iter()
        .filter(|ns| one_skip(1, ns.as_slice()) || one_skip(-1, ns.as_slice()))
        .count()
}

#[cfg(test)]
mod test {
    fn test_input() -> impl Iterator<Item = String> {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .lines()
            .map(|s| s.to_string())
    }

    #[test]
    fn test_day2_part1() {
        assert_eq!(2, super::day2_part1(test_input()))
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(4, super::day2_part2(test_input()))
    }
}
