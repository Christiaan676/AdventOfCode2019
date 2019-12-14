mod coordinate;
use coordinate::Coordinate;

use std::collections::HashMap;

fn main() {
    // Input program
    let mem = include_str!("program.txt");
    let mem: Vec<_> = mem.split(",").map(|s| s.parse().unwrap()).collect();

    let mut hull = Hull::default();
    Registers::new(&mem, &mut hull).run();

    println!("PART 1: Painted panels {}", hull.locations.len());

    let mut hull = Hull::default();
    hull.locations
        .insert(Coordinate::center(), PanelColor::WHITE);
    Registers::new(&mem, &mut hull).run();

    let paint_spots = hull.locations;

    let min_x = paint_spots.keys().map(|c| c.x).min().unwrap();
    let min_y = paint_spots.keys().map(|c| c.y).min().unwrap();

    let max_x = paint_spots.keys().map(|c| c.x).max().unwrap();
    let max_y = paint_spots.keys().map(|c| c.y).max().unwrap();

    println!("PART 2: Painted panels {}", paint_spots.len());
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match paint_spots.get(&Coordinate::new(x, y)) {
                Some(PanelColor::BLACK) => print!(" "),
                Some(PanelColor::WHITE) => print!("█"),
                None => print!(" "),
            }
        }
        println!("");
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
            Direction::RIGHT => Direction::UP,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }

    fn get_movement(&self) -> Coordinate {
        match self {
            Direction::UP => Coordinate::new(0, 1),
            Direction::DOWN => Coordinate::new(0, -1),
            Direction::LEFT => Coordinate::new(-1, 0),
            Direction::RIGHT => Coordinate::new(1, 0),
        }
    }
}

struct Hull {
    location: Coordinate,
    direction: Direction,
    state: State,
    locations: HashMap<Coordinate, PanelColor>,
}

impl Default for Hull {
    fn default() -> Self {
        Hull {
            location: Coordinate::center(),
            direction: Direction::UP,
            state: State::PAINT,
            locations: HashMap::new(),
        }
    }
}

enum State {
    PAINT,
    MOVE,
}

#[derive(Debug, Clone, Copy)]
enum PanelColor {
    WHITE,
    BLACK,
}

impl PanelColor {
    fn get_color_code(&self) -> isize {
        match self {
            PanelColor::BLACK => 0,
            PanelColor::WHITE => 1,
        }
    }

    fn from_color_code(i: isize) -> PanelColor {
        match i {
            0 => PanelColor::BLACK,
            1 => PanelColor::WHITE,
            _ => panic!("Unsupported color code"),
        }
    }
}

impl CpuIO for Hull {
    fn write(&mut self, output: isize) {
        self.state = match self.state {
            State::PAINT => {
                self.locations
                    .insert(self.location, PanelColor::from_color_code(output));
                State::MOVE
            }
            State::MOVE => {
                self.direction = match output {
                    0 => self.direction.rotate_left(),
                    1 => self.direction.rotate_right(),
                    _ => panic!("Unexpected move direction"),
                };
                self.location += self.direction.get_movement();

                State::PAINT
            }
        }
    }

    fn read(&mut self) -> isize {
        self.locations
            .get(&self.location)
            .unwrap_or(&PanelColor::BLACK)
            .get_color_code()
    }
}

trait CpuIO {
    fn write(&mut self, output: isize);
    fn read(&mut self) -> isize;
}

struct Registers<'a, I> {
    mem: Vec<isize>,
    pc: usize,
    relative_base: isize,
    mode: [u8; 3],
    opcode: u8,
    io_module: &'a mut I,
}

impl<'a, I: CpuIO> Registers<'a, I> {
    fn new(mem: &[isize], io_module: &'a mut I) -> Registers<'a, I> {
        let mut reg = Registers {
            mem: mem.to_vec(),
            pc: 0,
            relative_base: 0,
            mode: [0; 3],
            opcode: 0,
            io_module,
        };
        reg.decode();
        reg
    }

    fn decode(&mut self) {
        let mut instruction = self.mem[self.pc];

        self.opcode = (instruction % 100) as u8;
        instruction /= 100;
        for reg in 0..=2 {
            self.mode[reg] = (instruction % 10) as u8;
            instruction /= 10;
        }
    }

    fn get_arg(&self, i: usize) -> isize {
        match self.mode[i - 1] {
            0 => self.read(self.read(self.pc + i) as usize), // Position / pointer mode
            1 => self.read(self.pc + i),                     // Paramter mode
            2 => self.read((self.relative_base + self.read(self.pc + i)) as usize), // Relative mode
            _ => panic!("Unsupported mode"),
        }
    }

    fn read(&self, index: usize) -> isize {
        *self.mem.get(index).unwrap_or(&0)
    }

    fn write(&mut self, i: usize, value: isize) {
        match self.mode[i - 1] {
            0 => self.write_mem(self.read(self.pc + i) as usize, value), // Position / pointer mode
            1 => panic!("Mode not supported for writes"),                // Paramter mode
            2 => self.write_mem(
                (self.relative_base + self.read(self.pc + i)) as usize,
                value,
            ), // Relative mode
            _ => panic!("Unsupported mode"),
        }
    }

    fn write_mem(&mut self, location: usize, value: isize) {
        if self.mem.len() <= location {
            // Expand memmory
            self.mem.resize(location + 1, 0);
        }
        self.mem[location] = value;
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }

    fn goto(&mut self, new_pc: usize) {
        self.pc = new_pc;
        self.decode();
    }

    fn increment_pc(&mut self, i: usize) {
        self.pc += i;
        self.decode();
    }

    // Run untill output is produced or halted
    fn run(&mut self) {
        loop {
            match self.get_opcode() {
                1 => {
                    // add
                    self.write(3, self.get_arg(1) + self.get_arg(2));
                    self.increment_pc(4);
                }
                2 => {
                    // mull
                    self.write(3, self.get_arg(1) * self.get_arg(2));
                    self.increment_pc(4);
                }
                3 => {
                    // Input
                    let input = self.io_module.read();
                    self.write(1, input);
                    self.increment_pc(2);
                }
                4 => {
                    // Ouput
                    self.io_module.write(self.get_arg(1));
                    self.increment_pc(2);
                }
                5 => {
                    //jump-if-true
                    if self.get_arg(1) != 0 {
                        self.goto(self.get_arg(2) as usize);
                    } else {
                        self.increment_pc(3);
                    }
                }
                6 => {
                    //jump-if-false
                    if self.get_arg(1) == 0 {
                        self.goto(self.get_arg(2) as usize);
                    } else {
                        self.increment_pc(3);
                    }
                }
                7 => {
                    //less than
                    if self.get_arg(1) < self.get_arg(2) {
                        self.write(3, 1);
                    } else {
                        self.write(3, 0);
                    }
                    self.increment_pc(4);
                }
                8 => {
                    //equals
                    if self.get_arg(1) == self.get_arg(2) {
                        self.write(3, 1);
                    } else {
                        self.write(3, 0);
                    }
                    self.increment_pc(4);
                }
                9 => {
                    // Update relative base
                    self.relative_base = self.relative_base + self.get_arg(1);
                    self.increment_pc(2);
                }
                99 => {
                    // Halt
                    break;
                }
                _ => {
                    // Somethings wronge
                    panic!("Unsupported opcode");
                }
            }
        }
    }
}
