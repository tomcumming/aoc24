pub fn day2(part_two: bool) {
    let ls = std::io::stdin().lines().map(|l| l.unwrap());
    let res = if part_two {
        todo!();
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

fn pairs<'a, A: Sized + Clone>(xs: &'a [A]) -> impl 'a + Iterator<Item = (A, A)> {
    (0..(xs.len() - 1)).map(move |i| (xs[i].clone(), xs[i + 1].clone()))
}

fn safe(dir: i32, ns: &[i32]) -> bool {
    pairs(ns).map(|(x, y)| (x - y)).all(|delta| {
        let m = i32::abs(delta);
        i32::signum(delta) == dir && m <= 3 && m >= 1
    })
}

fn day2_part1(ls: impl Iterator<Item = String>) -> i32 {
    parse_input(ls)
        .into_iter()
        .filter(|ns| safe(1, ns) || safe(-1, ns))
        .count() as i32
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
}
