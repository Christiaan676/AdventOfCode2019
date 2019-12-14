use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::ops;

use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let mut astroids = to_coordinate_set(&input);
    let (monitor_station, visible) = calc_best_astroid(&astroids);
    println!(
        "PART 1:  astroid: {:?} can see: {}",
        monitor_station, visible
    );

    astroids.remove(&monitor_station);
    let kill_order = calc_kill_order(monitor_station, &astroids);

    let astroid_200th = kill_order.get(199).unwrap();
    println!(
        "PART 2:  200th astroid: {:?} awnser: {}",
        astroid_200th,
        astroid_200th.x * 100 + astroid_200th.y
    );
}

fn to_coordinate_set(input: &str) -> HashSet<Coordinate> {
    (0..)
        .zip(input.lines())
        //.enumarate() not sure why enumarate is not availible, type unsized?
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Coordinate::new(x as i32, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn calc_best_astroid(asteroids: &HashSet<Coordinate>) -> (Coordinate, usize) {
    asteroids
        .iter()
        .map(|astoid| (astoid, count_visible(*astoid, &asteroids)))
        .fold((Coordinate::center(), 0), |(astoid, count), (a, c)| {
            if c > count {
                (*a, c)
            } else {
                (astoid, count)
            }
        })
}

fn count_visible(origing: Coordinate, astroids: &HashSet<Coordinate>) -> usize {
    let mut visible = HashMap::new();
    for &astroid in astroids {
        if astroid == origing {
            continue;
        }
        let relative = astroid - origing;

        visible.insert(relative.angle(), astroid);
    }
    visible.len()
}

fn calc_kill_order(
    monitor_station: Coordinate,
    asteroids: &HashSet<Coordinate>,
) -> Vec<Coordinate> {
    let mut asteroids = asteroids.clone();
    let mut result = vec![];

    while !asteroids.is_empty() {
        let mut pass = calc_single_pass(monitor_station, &asteroids);
        for a in pass.iter() {
            asteroids.remove(&a);
        }
        result.append(&mut pass);
    }
    result
}

fn calc_single_pass(
    monitor_station: Coordinate,
    asteroids: &HashSet<Coordinate>,
) -> Vec<Coordinate> {
    let mut results = HashMap::new();
    for &astroid in asteroids {
        let relative_cordinate = astroid - monitor_station;

        match results.entry(relative_cordinate.angle()) {
            Entry::Occupied(mut entry) => {
                // Keep the astroud that is closest by
                let entry_relative: Coordinate = *entry.get() - monitor_station;
                if entry_relative.length() > relative_cordinate.length() {
                    entry.insert(astroid);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(astroid);
            }
        }
    }
    let mut sorted_vec: Vec<_> = results
        .values()
        .map(|&a| SortCoordinate::new(monitor_station, a))
        .collect();
    sorted_vec.sort();
    println!("Pass {} {:?}", sorted_vec.len(), sorted_vec);
    let result: Vec<_> = sorted_vec.iter().map(|a| a.coordinate).collect();

    result
}

// Sorted coordinates by angle
#[derive(Eq, Debug)]
struct SortCoordinate {
    angle: usize,
    coordinate: Coordinate,
}

impl Ord for SortCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle.cmp(&other.angle)
    }
}

impl PartialOrd for SortCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SortCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.angle == other.angle
    }
}

impl SortCoordinate {
    fn new(origin: Coordinate, coordinate: Coordinate) -> SortCoordinate {
        let relative = coordinate - origin;
        SortCoordinate {
            angle: relative.angle(),
            coordinate,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coordinate {
    fn center() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    // We only need a lengt for comparison so don't bother with sqrt
    fn length(self) -> usize {
        (self.x.pow(2) + self.y.pow(2)) as usize
    }

    // Angle relative to Up
    fn angle(self) -> usize {
        let mut angle = (self.y as f64).atan2(self.x as f64).to_degrees();
        // Remove negative part and rotate angel 90 degrees
        angle = (angle + 360.0 + 90.0) % 360.0;

        (angle * 100.0).round() as usize
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_module_fuel_test() {
        assert_eq!(Coordinate::new(0, -1).angle(), 0);
        assert_eq!(Coordinate::new(1, 0).angle(), 9000);
        assert_eq!(Coordinate::new(0, 1).angle(), 18000);
        assert_eq!(Coordinate::new(-1, 0).angle(), 27000);
    }
}
