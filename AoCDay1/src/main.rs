use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(&input_file);
    let total_fuel: u32 = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .map(calc_module_fuel)
        .sum();
    println!("Total fuel: {}", total_fuel);

    Ok(())
}

fn calc_module_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_module_fuel_test() {
        // Input and results taken from example
        assert_eq!(calc_module_fuel(12), 2);
        assert_eq!(calc_module_fuel(14), 2);
        assert_eq!(calc_module_fuel(1969), 654);
        assert_eq!(calc_module_fuel(100756), 33583);
    }
}
