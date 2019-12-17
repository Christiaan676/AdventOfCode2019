mod point;
use point::Point;

use std::collections::HashSet;

fn main() {
    // Input program
    let mem = include_str!("input.txt");
    let mem: Vec<_> = mem.split(",").map(|s| s.parse().unwrap()).collect();

    let mut game_state = GameState::default();
    Registers::new(&mem, &mut game_state).run();

    //println!("PART 1: movements: {}", );
    game_state.print();

    let field = game_state.display;
    let mut distance: [[u32; 50]; 50] = [[std::u32::MAX; 50]; 50];
    let mut eval_points = HashSet::new();

    distance[25][25] = 0; // Set start point to 0 distance
    eval_points.insert((25, 25));
    let mut short_distance = 0;
    while !eval_points.is_empty() {
        let (x_low, y_low) = get_lowest_point(&mut eval_points, &distance);
        let value = distance[y_low][x_low];

        if field[y_low][x_low] == GameTile::Oxygen {
            short_distance = value;
        }

        // Fill in ajecent distances
        if let Some(p) = update_point(&field, &mut distance, (x_low + 1, y_low), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low - 1, y_low), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low, y_low + 1), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low, y_low - 1), value + 1) {
            eval_points.insert(p);
        }
    }
    println!("PART 1: {}", short_distance);

    let (oxy_x, oxy_y) = game_state.get_oxygen_location();

    println!("Oxygen location; {} {}", oxy_x, oxy_y);

    let field = game_state.display;
    let mut distance: [[u32; 50]; 50] = [[std::u32::MAX; 50]; 50];
    let mut eval_points = HashSet::new();

    distance[oxy_y][oxy_x] = 0; // Set start point to 0 distance
    eval_points.insert((oxy_x, oxy_y));
    while !eval_points.is_empty() {
        let (x_low, y_low) = get_lowest_point(&mut eval_points, &distance);
        let value = distance[y_low][x_low];
        //println!("PART 2: {} {}  {}", x_low, y_low, value);
        // Fill in ajecent distances
        if let Some(p) = update_point(&field, &mut distance, (x_low + 1, y_low), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low - 1, y_low), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low, y_low + 1), value + 1) {
            eval_points.insert(p);
        }
        if let Some(p) = update_point(&field, &mut distance, (x_low, y_low - 1), value + 1) {
            eval_points.insert(p);
        }
    }

    let time = distance
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&v| v < std::u32::MAX)
        .max()
        .unwrap();

    println!("PART 2: max: {}", time);
}

fn update_point(
    field: &[[GameTile; 50]; 50],
    distance: &mut [[u32; 50]; 50],
    (x, y): (usize, usize),
    value: u32,
) -> Option<(usize, usize)> {
    if distance[y][x] == std::u32::MAX
        && (field[y][x] == GameTile::PATH || field[y][x] == GameTile::Oxygen)
    {
        distance[y][x] = value;
        Some((x, y))
    } else {
        None
    }
}

fn get_lowest_point(
    eval_points: &mut HashSet<(usize, usize)>,
    distance: &[[u32; 50]; 50],
) -> (usize, usize) {
    let fold_func = |(point, dist): ((usize, usize), u32), (p, d): ((usize, usize), u32)| {
        if d < dist {
            (p, d)
        } else {
            (point, dist)
        }
    };
    let inital: ((usize, usize), u32) = ((0, 0), std::u32::MAX);
    let (p, _c) = eval_points
        .iter()
        .map(|&(x, y): &(usize, usize)| ((x, y), distance[y][x]))
        .fold(inital, fold_func);
    eval_points.remove(&p);
    p
}

struct GameState {
    position: Point,
    path: Vec<MoveMent>,
    display: [[GameTile; 50]; 50],
}

impl Default for GameState {
    fn default() -> Self {
        let mut state = GameState {
            position: Point::new(25, 25),
            path: Vec::new(), // Movements that lead to the current position
            display: [[GameTile::Unknow; 50]; 50],
        };
        state.update_position(GameTile::PATH);
        state
    }
}

impl GameState {
    fn update_position(&mut self, tile_state: GameTile) {
        self.display[self.position.y as usize][self.position.x as usize] = tile_state;
    }

    fn get_tile(&self, position: Point) -> GameTile {
        self.display[position.y as usize][position.x as usize]
    }

    fn get_move(&self) -> Option<MoveMent> {
        [
            MoveMent::North,
            MoveMent::East,
            MoveMent::South,
            MoveMent::West,
        ]
        .iter()
        .find(|move_ment| {
            self.get_tile(self.position + move_ment.get_move_vec()) == GameTile::Unknow
        })
        .cloned()
    }

    fn print(&self) {
        for (y, line) in self.display.iter().enumerate().rev() {
            for (x, c) in line.iter().enumerate() {
                if Point::new(x as i32, y as i32) == self.position {
                    print!("*");
                    continue;
                }
                match c {
                    GameTile::Unknow => print!("?"),
                    GameTile::WALL => print!("#"),
                    GameTile::PATH => print!(" "),
                    GameTile::Oxygen => print!("o"),
                }
            }
            println!("");
        }
    }

    fn get_oxygen_location(&self) -> (usize, usize) {
        for (y, line) in self.display.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == GameTile::Oxygen {
                    return (x, y);
                }
            }
        }
        panic!();
    }
}

impl CpuIO for GameState {
    fn write(&mut self, output: isize) {
        //println!("{:?}", Status::from_status_code(output));
        match Status::from_status_code(output) {
            Status::HitWall => {
                self.update_position(GameTile::WALL);

                // Rollback step as it failed
                let removed_step = self.path.pop().unwrap();
                self.position -= removed_step.get_move_vec();
            }
            Status::StepOk => {
                self.update_position(GameTile::PATH);
            }
            Status::StepOkOxygen => {
                self.update_position(GameTile::Oxygen);
            }
        }
    }

    fn read(&mut self) -> isize {
        // use std::io::{self, Read};

        // let mut input = String::new();
        // io::stdin().read_line(&mut input);

        //self.print();
        // Move to unexplorerd tile
        if let Some(move_ment) = self.get_move() {
            self.position += move_ment.get_move_vec();
            self.path.push(move_ment);
            //println!("Move: {:?}", move_ment);
            return move_ment as isize;
        }

        // If not unexplorerd track back one step
        if let Some(move_ment) = self.path.pop() {
            //println!("Backtrack: {:?}", move_ment);
            self.position -= move_ment.get_move_vec();
            return move_ment.get_reverse_move() as isize;
        }

        // If no step back all explored!
        0 // Stops the program
    }
}

/**
 * 0 is an empty tile. No game object appears in this tile.
 * 1 is a wall tile. Walls are indestructible barriers.
 * 2 is a block tile. Blocks can be broken by the ball.
 * 3 is a horizontal paddle tile. The paddle is indestructible.
 * 4 is a ball tile. The ball moves diagonally and bounces off objects.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameTile {
    Unknow,
    WALL,
    PATH,
    Oxygen,
}

//north (1), south (2), west (3), and east (4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveMent {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl MoveMent {
    fn get_move_vec(&self) -> Point {
        match self {
            MoveMent::North => Point::new(0, 1),
            MoveMent::South => Point::new(0, -1),
            MoveMent::West => Point::new(-1, 0),
            MoveMent::East => Point::new(1, 0),
        }
    }

    // The move to undo this move
    fn get_reverse_move(&self) -> MoveMent {
        match self {
            MoveMent::North => MoveMent::South,
            MoveMent::South => MoveMent::North,
            MoveMent::West => MoveMent::East,
            MoveMent::East => MoveMent::West,
        }
    }
}

// 0: The repair droid hit a wall. Its position has not changed.
// 1: The repair droid has moved one step in the requested direction.
// 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    HitWall,
    StepOk,
    StepOkOxygen,
}

impl Status {
    fn from_status_code(output: isize) -> Status {
        match output {
            0 => Status::HitWall,
            1 => Status::StepOk,
            2 => Status::StepOkOxygen,
            _ => panic!("Unknown statuscode "),
        }
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
