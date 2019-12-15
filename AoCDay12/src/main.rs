extern crate num;

use num::integer::lcm;

mod point;
use point::Point;

fn main() {
    // <x=-13, y=-13, z=-13>
    // <x=  5, y= -8, z=  3>
    // <x= -6, y=-10, z= -3>
    // <x=  0, y=  5, z= -5>
    let moons = vec![
        Moon::new(-13, -13, -13),
        Moon::new(5, -8, 3),
        Moon::new(-6, -10, -3),
        Moon::new(0, 5, -5),
    ];

    // <x=-1, y=  0, z= 2>
    // <x= 2, y=-10, z=-7>
    // <x= 4, y= -8, z= 8>
    // <x= 3, y=  5, z=-1>
    // let moons = vec!(
    //     Moon::new(-1,   0,  2),
    //     Moon::new( 2, -10, -7),
    //     Moon::new( 4,  -8,  8),
    //     Moon::new( 3,   5, -1)
    // );

    // <x=-8, y=-10, z=0>
    // <x= 5, y=5, z=10>
    // <x= 2, y=-7, z=3>
    // <x= 9, y=-8, z=-3>
    // let moons = vec!(
    //     Moon::new(-8,-10, 0),
    //     Moon::new( 5,  5, 10),
    //     Moon::new( 2, -7, 3),
    //     Moon::new( 9, -8, -3)
    // );

    let mut space = Space {
        moons: moons.clone(),
    };
    for _ in 0..1000 {
        space.time_step();
    }
    println!("PART1: energy {}", space.total_energy());

    let mut space = Space {
        moons: moons.clone(),
    };

    let dim_start_state: Vec<_> = (0..3).map(|dim| dimention_state(&moons, dim)).collect();

    let mut phase = vec![0_usize; 3];
    let mut step: usize = 0;
    while phase.iter().any(|&v| v == 0) {
        space.time_step();
        step += 1;

        for dim in 0..3 {
            if phase[dim] == 0 && dimention_state(&space.moons, dim) == dim_start_state[dim] {
                phase[dim] = step;
            }
        }
    }

    println!("Phases: {:?}", phase);
    let lcm = phase
        .iter()
        .skip(2)
        .fold(lcm(phase[0], phase[1]), |acc, v| lcm(acc, *v));

    println!("PART2: {:?} {}", phase, lcm);
}

fn dimention_state(moons: &[Moon], dim: usize) -> Vec<i32> {
    moons
        .iter()
        .flat_map(|m| m.dim(dim).to_vec().into_iter())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Point::new(x, y, z),
            velocity: Point::default(),
        }
    }

    fn upate_velocity(&mut self, other_moon: &Point) {
        self.velocity.x += (other_moon.x - self.position.x).signum();
        self.velocity.y += (other_moon.y - self.position.y).signum();
        self.velocity.z += (other_moon.z - self.position.z).signum();
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn dim(&self, dim: usize) -> [i32; 2] {
        match dim {
            0 => [self.position.x, self.velocity.x],
            1 => [self.position.y, self.velocity.y],
            2 => [self.position.z, self.velocity.z],
            _ => panic!(),
        }
    }
}

struct Space {
    moons: Vec<Moon>,
}

impl Space {
    fn time_step(&mut self) {
        // Update all velocity
        for moon_a in 0..self.moons.len() {
            for moon_b in 0..self.moons.len() {
                if moon_a == moon_b {
                    continue;
                }
                let moon_b_pos = self.moons[moon_b].position;
                self.moons[moon_a].upate_velocity(&moon_b_pos);
            }
        }

        // Update all positions
        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }

    fn total_energy(&self) -> i32 {
        self.moons
            .iter()
            .map(|m| m.potential_energy() * m.kinetic_energy())
            .sum()
    }
}
