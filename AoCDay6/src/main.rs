use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    // Build MAP planet -> Moons
    let mut planets: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (planet, moon) = parse_line(line);
        planets.entry(planet).or_default().push(moon);
    }

    //Walk tree
    let total_orbits = orbits(&planets, "COM", 0);
    println!("PART 1: Orbits: {}", total_orbits);

    // Build MAP planet -> Moons
    let mut moons: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        let (planet, moon) = parse_line(line);
        moons.insert(moon, planet);
    }

    let mut you_path = path_to_com(&moons, "YOU");
    let mut san_path = path_to_com(&moons, "SAN");

    loop {
        let you = you_path.pop().unwrap();
        let san = san_path.pop().unwrap();
        if you != san {
            you_path.push(you);
            san_path.push(san);
            break;
        }
    }

    println!("PART2: {} ", you_path.len() + san_path.len());
}

fn parse_line(line: &str) -> (String, String) {
    let mut it = line.split(')');
    (
        it.next().unwrap().to_string(),
        it.next().unwrap().to_string(),
    )
}

fn orbits(planets: &HashMap<String, Vec<String>>, planet: &str, current_orit: usize) -> usize {
    let mut total_orbits = current_orit; // Add own distance
    if let Some(my_moons) = planets.get(planet) {
        for moon in my_moons {
            total_orbits += orbits(planets, moon, current_orit + 1);
        }
    }
    total_orbits
}

fn path_to_com(moons: &HashMap<String, String>, start: &str) -> Vec<String> {
    let mut path = vec![];
    let mut current = start;
    loop {
        if let Some(parent) = moons.get(current) {
            current = parent;
            path.push(parent.to_string());
        } else {
            break;
        }
    }
    path
}
