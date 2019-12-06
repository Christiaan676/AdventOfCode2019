fn main() {
    let result: Vec<u32> = (172851..=675869)
        .into_iter()
        .filter(|value| filter_part1(*value))
        .collect();

    println!(
        "PART1: Number of options: {:?} len: {}",
        result,
        result.len()
    );

    let result: Vec<u32> = (172851..=675869)
        .into_iter()
        .filter(|value| filter_part2(*value))
        .collect();

    println!(
        "PART2: Number of options: {:?} len: {}",
        result,
        result.len()
    );
}

fn filter_part2(value: u32) -> bool {
    // Check if all values are increasing and contains atlease on double value
    let string_rep = value.to_string();
    let mut double = false;
    let mut same = 1;
    let mut last_char = None;

    for c in string_rep.chars() {
        if let Some(last) = last_char {
            if last == c {
                same += 1;
            } else {
                if same == 2 {
                    double = true;
                }
                same = 1;
            }
            if c < last {
                return false;
            }
        }
        last_char = Some(c);
    }
    // Neede if the two chars are at the end
    if same == 2 {
        double = true;
    }

    double
}

fn filter_part1(value: u32) -> bool {
    // Check if all values are increasing and contains atlease on double value
    let string_rep = value.to_string();
    let mut double = false;
    let mut last_char = None;

    for c in string_rep.chars() {
        if let Some(last) = last_char {
            if last == c {
                double = true;
            }
            if c < last {
                return false;
            }
        }
        last_char = Some(c);
    }
    double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input and results taken from example
        assert_eq!(filter_part1(111111), true);
    }

    #[test]
    fn test2() {
        // Input and results taken from example
        assert_eq!(filter_part1(223450), false);
    }

    #[test]
    fn test3() {
        // Input and results taken from example
        assert_eq!(filter_part1(123789), false);
    }

    #[test]
    fn test4() {
        assert_eq!(filter_part1(177777), true);
    }

    #[test]
    fn test5() {
        // Input and results taken from example
        assert_eq!(filter_part2(112233), true);
    }

    #[test]
    fn test_part2_1() {
        // Input and results taken from example
        assert_eq!(filter_part2(112233), true);
    }

    #[test]
    fn test_part2_2() {
        // Input and results taken from example
        assert_eq!(filter_part2(123444), false);
    }

    #[test]
    fn test_part2_3() {
        // Input and results taken from example
        assert_eq!(filter_part2(111122), true);
    }
}
