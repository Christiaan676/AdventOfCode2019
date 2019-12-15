fn main() {
    // Input program
    let mem = include_str!("input.txt");
    let mut mem: Vec<_> = mem.split(",").map(|s| s.parse().unwrap()).collect();

    let mut display = GameDisplay::default();
    Registers::new(&mem, &mut display).run();

    let nr_block_tiles = display
        .display
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&tile| tile == GameTile::BLOCK)
        .count();

    println!("PART 1: NR Block tiles: {}", nr_block_tiles);

    // Play for free
    mem[0] = 2;

    let mut display = GameDisplay::default();
    Registers::new(&mem, &mut display).run();

    println!("PART 2: Score: {}", display.score);
}

struct GameDisplay {
    state: State,
    score: usize,
    display: [[GameTile; 200]; 200],
}

impl Default for GameDisplay {
    fn default() -> Self {
        GameDisplay {
            state: State::XPos,
            score: 0,
            display: [[GameTile::EMPTY; 200]; 200],
        }
    }
}

impl GameDisplay {
    fn get_pos_ball(&self) -> (usize, usize) {
        self.get_pos(GameTile::BALL)
    }

    fn get_pos_paddle(&self) -> (usize, usize) {
        self.get_pos(GameTile::PADDLE)
    }

    fn get_pos(&self, tile_type: GameTile) -> (usize, usize) {
        for x in 0..200 {
            for y in 0..200 {
                if self.display[x][y] == tile_type {
                    return (x, y);
                }
            }
        }
        unimplemented!()
    }
}

enum State {
    XPos,
    YPos(isize),
    TileID(isize, isize),
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
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

impl GameTile {
    fn form_isize(input: isize) -> GameTile {
        match input {
            0 => GameTile::EMPTY,
            1 => GameTile::WALL,
            2 => GameTile::BLOCK,
            3 => GameTile::PADDLE,
            4 => GameTile::BALL,
            _ => panic!("Unkown tile type"),
        }
    }
}

impl CpuIO for GameDisplay {
    fn write(&mut self, output: isize) {
        self.state = match self.state {
            State::XPos => State::YPos(output),
            State::YPos(x) => State::TileID(x, output),
            State::TileID(x, y) => {
                if x == -1 && y == 0 {
                    self.score = output as usize;
                } else {
                    self.display[x as usize][y as usize] = GameTile::form_isize(output);
                }
                State::XPos
            }
        }
    }

    fn read(&mut self) -> isize {
        // If the joystick is in the neutral position, provide 0.
        // If the joystick is tilted to the left, provide -1.
        // If the joystick is tilted to the right, provide
        let (ball_x, ball_y) = self.get_pos_ball();
        let (paddle_x, paddle_y) = self.get_pos_paddle();

        // If  paddle is not under ball move it there
        (ball_x as isize - paddle_x as isize).signum()
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
