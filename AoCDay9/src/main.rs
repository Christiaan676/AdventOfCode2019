use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    // Input program
    let mem = vec![
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1102, 1, 24, 1017, 1101, 0, 36, 1006, 1101, 0, 30, 1011, 1101, 26,
        0, 1018, 1101, 32, 0, 1015, 1101, 34, 0, 1004, 1101, 0, 37, 1002, 1101, 25, 0, 1012, 1102,
        38, 1, 1010, 1101, 29, 0, 1019, 1101, 308, 0, 1029, 1102, 1, 696, 1027, 1102, 1, 429, 1022,
        1102, 1, 21, 1005, 1102, 1, 33, 1013, 1101, 39, 0, 1008, 1102, 20, 1, 1009, 1101, 0, 652,
        1025, 1102, 313, 1, 1028, 1101, 0, 31, 1003, 1102, 661, 1, 1024, 1101, 35, 0, 1016, 1101,
        0, 23, 1000, 1102, 28, 1, 1014, 1102, 0, 1, 1020, 1102, 27, 1, 1007, 1101, 0, 1, 1021,
        1102, 22, 1, 1001, 1101, 703, 0, 1026, 1101, 0, 422, 1023, 109, -5, 2101, 0, 9, 63, 1008,
        63, 31, 63, 1005, 63, 205, 1001, 64, 1, 64, 1105, 1, 207, 4, 187, 1002, 64, 2, 64, 109, 6,
        2102, 1, 3, 63, 1008, 63, 37, 63, 1005, 63, 227, 1105, 1, 233, 4, 213, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, 11, 21108, 40, 40, 3, 1005, 1015, 255, 4, 239, 1001, 64, 1, 64, 1106,
        0, 255, 1002, 64, 2, 64, 109, -3, 21107, 41, 40, 2, 1005, 1011, 275, 1001, 64, 1, 64, 1105,
        1, 277, 4, 261, 1002, 64, 2, 64, 109, 4, 2107, 28, -6, 63, 1005, 63, 297, 1001, 64, 1, 64,
        1106, 0, 299, 4, 283, 1002, 64, 2, 64, 109, 15, 2106, 0, 0, 4, 305, 1106, 0, 317, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, -23, 2108, 22, 4, 63, 1005, 63, 337, 1001, 64, 1, 64, 1105, 1,
        339, 4, 323, 1002, 64, 2, 64, 109, 6, 21101, 42, 0, 0, 1008, 1011, 40, 63, 1005, 63, 363,
        1001, 64, 1, 64, 1105, 1, 365, 4, 345, 1002, 64, 2, 64, 109, -17, 1207, 7, 21, 63, 1005,
        63, 381, 1105, 1, 387, 4, 371, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 1201, -1, 0, 63,
        1008, 63, 25, 63, 1005, 63, 407, 1105, 1, 413, 4, 393, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, 15, 2105, 1, 0, 1001, 64, 1, 64, 1105, 1, 431, 4, 419, 1002, 64, 2, 64, 109, -23,
        2101, 0, 6, 63, 1008, 63, 36, 63, 1005, 63, 453, 4, 437, 1106, 0, 457, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, 10, 2108, 21, -5, 63, 1005, 63, 475, 4, 463, 1106, 0, 479, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, -3, 1201, 2, 0, 63, 1008, 63, 20, 63, 1005, 63, 505, 4, 485,
        1001, 64, 1, 64, 1105, 1, 505, 1002, 64, 2, 64, 109, 4, 2107, 35, -5, 63, 1005, 63, 527, 4,
        511, 1001, 64, 1, 64, 1105, 1, 527, 1002, 64, 2, 64, 109, 15, 1206, -5, 543, 1001, 64, 1,
        64, 1105, 1, 545, 4, 533, 1002, 64, 2, 64, 109, -8, 1205, 3, 563, 4, 551, 1001, 64, 1, 64,
        1106, 0, 563, 1002, 64, 2, 64, 109, -5, 1206, 7, 581, 4, 569, 1001, 64, 1, 64, 1105, 1,
        581, 1002, 64, 2, 64, 109, -8, 1207, -3, 38, 63, 1005, 63, 599, 4, 587, 1105, 1, 603, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 19, 1205, -4, 619, 1001, 64, 1, 64, 1105, 1, 621, 4, 609,
        1002, 64, 2, 64, 109, -13, 1208, -4, 27, 63, 1005, 63, 639, 4, 627, 1105, 1, 643, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, 5, 2105, 1, 8, 4, 649, 1001, 64, 1, 64, 1106, 0, 661, 1002,
        64, 2, 64, 109, -16, 1202, 4, 1, 63, 1008, 63, 34, 63, 1005, 63, 683, 4, 667, 1106, 0, 687,
        1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26, 2106, 0, 1, 1001, 64, 1, 64, 1105, 1, 705, 4,
        693, 1002, 64, 2, 64, 109, -9, 21102, 43, 1, -7, 1008, 1010, 46, 63, 1005, 63, 725, 1105,
        1, 731, 4, 711, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -26, 1202, 9, 1, 63, 1008, 63, 26,
        63, 1005, 63, 755, 1001, 64, 1, 64, 1105, 1, 757, 4, 737, 1002, 64, 2, 64, 109, 34, 21108,
        44, 43, -8, 1005, 1017, 773, 1106, 0, 779, 4, 763, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
        -15, 21102, 45, 1, 1, 1008, 1011, 45, 63, 1005, 63, 801, 4, 785, 1106, 0, 805, 1001, 64, 1,
        64, 1002, 64, 2, 64, 109, -14, 1208, 10, 35, 63, 1005, 63, 821, 1106, 0, 827, 4, 811, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 17, 2102, 1, -4, 63, 1008, 63, 20, 63, 1005, 63, 853, 4,
        833, 1001, 64, 1, 64, 1106, 0, 853, 1002, 64, 2, 64, 109, 6, 21107, 46, 47, -4, 1005, 1015,
        871, 4, 859, 1105, 1, 875, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 21101, 47, 0, 4,
        1008, 1013, 47, 63, 1005, 63, 901, 4, 881, 1001, 64, 1, 64, 1105, 1, 901, 4, 64, 99, 21102,
        27, 1, 1, 21102, 1, 915, 0, 1106, 0, 922, 21201, 1, 37790, 1, 204, 1, 99, 109, 3, 1207, -2,
        3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102, 1, 942, 0, 1106, 0, 922, 22102, 1, 1, -1,
        21201, -2, -3, 1, 21102, 957, 1, 0, 1105, 1, 922, 22201, 1, -1, -2, 1105, 1, 968, 21201,
        -2, 0, -2, 109, -3, 2105, 1, 0,
    ];

    let result = Registers::new(&mem, &[1]).run();
    println!("PART 1: BOOST: {:?}", result);

    let result = Registers::new(&mem, &[2]).run();
    println!("PART 1: Coordinates : {:?}", result);
}

struct Registers {
    mem: Vec<isize>,
    pc: usize,
    relative_base: isize,
    mode: [u8; 3],
    opcode: u8,
    input: VecDeque<isize>,
}

impl Registers {
    fn new(mem: &[isize], input: &[isize]) -> Registers {
        let mut reg = Registers {
            mem: mem.to_vec(),
            pc: 0,
            relative_base: 0,
            mode: [0; 3],
            opcode: 0,
            input: VecDeque::from_iter(input.iter().cloned()),
        };
        reg.decode();
        reg
    }

    fn add_input(&mut self, input: isize) {
        self.input.push_back(input);
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

    fn run(&mut self) -> Vec<isize> {
        let mut output = vec![];
        while let Some(out) = self.step() {
            output.push(out);
        }
        output
    }

    // Run untill output is produced or halted
    fn step(&mut self) -> Option<isize> {
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
                    let var = self.input.pop_front().expect("Expected input!");
                    self.write(1, var);
                    self.increment_pc(2);
                }
                4 => {
                    // Ouput
                    let output = self.get_arg(1);
                    self.increment_pc(2);
                    break Some(output);
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
                    break None;
                }
                _ => {
                    // Somethings wronge
                    panic!("Unsupported opcode");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examle_day2_1() {
        // Input and results taken from example
        let mem = vec![1, 0, 0, 0, 99];
        let mut cpu = Registers::new(&mem, &[]);
        cpu.run();

        assert_eq!(&[2, 0, 0, 0, 99], cpu.mem.as_slice());
    }

    #[test]
    fn examle_day2_2() {
        // Input and results taken from example
        let mem = vec![2, 3, 0, 3, 99];
        let mut cpu = Registers::new(&mem, &[]);
        cpu.run();

        assert_eq!(&[2, 3, 0, 6, 99], cpu.mem.as_slice());
    }

    #[test]
    fn examle_day2_3() {
        // Input and results taken from example
        let mem = vec![2, 4, 4, 5, 99, 0];
        let mut cpu = Registers::new(&mem, &[]);
        cpu.run();

        assert_eq!(&[2, 4, 4, 5, 99, 9801], cpu.mem.as_slice());
    }

    #[test]
    fn examle_day2_4() {
        // Input and results taken from example
        let mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut cpu = Registers::new(&mem, &[]);
        cpu.run();

        assert_eq!(&[30, 1, 1, 4, 2, 5, 6, 0, 99], cpu.mem.as_slice());
    }

    #[test]
    fn examle_day5_1() {
        // Input and results taken from example
        let mem = vec![3, 0, 4, 0, 99];
        let mut cpu = Registers::new(&mem, &[5678]);
        let output = cpu.run();

        assert_eq!(&[5678, 0, 4, 0, 99], cpu.mem.as_slice());
        assert_eq!(&[5678], output.as_slice());
    }

    #[test]
    fn examle_day5_2() {
        // Input and results taken from example
        let mem = vec![1101, 100, -1, 4, 0];
        let mut cpu = Registers::new(&mem, &[5678]);
        let output = cpu.run();

        assert_eq!(&[1101, 100, -1, 4, 99], cpu.mem.as_slice());
        assert!(output.is_empty());
    }

    #[test]
    fn examle_day5_part2_1a() {
        // Input and results taken from example
        let mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Registers::new(&mem, &[8]);
        let output = cpu.run();

        assert_eq!(&[3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], cpu.mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_1b() {
        // Input and results taken from example
        let mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Registers::new(&mem, &[18]);
        let output = cpu.run();

        assert_eq!(&[3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8], cpu.mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_2a() {
        // Input and results taken from example
        let mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Registers::new(&mem, &[5]);
        let output = cpu.run();

        assert_eq!(&[3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8], cpu.mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_2b() {
        // Input and results taken from example
        let mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Registers::new(&mem, &[18]);
        let output = cpu.run();

        assert_eq!(&[3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], cpu.mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_3a() {
        // Input and results taken from example
        let mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut cpu = Registers::new(&mem, &[8]);
        let output = cpu.run();

        assert_eq!(&[3, 3, 1108, 1, 8, 3, 4, 3, 99], cpu.mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_3b() {
        // Input and results taken from example
        let mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut cpu = Registers::new(&mem, &[18]);
        let output = cpu.run();

        assert_eq!(&[3, 3, 1108, 0, 8, 3, 4, 3, 99], cpu.mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_4a() {
        // Input and results taken from example
        let mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut cpu = Registers::new(&mem, &[5]);
        let output = cpu.run();

        assert_eq!(&[3, 3, 1107, 1, 8, 3, 4, 3, 99], cpu.mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_4b() {
        // Input and results taken from example
        let mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut cpu = Registers::new(&mem, &[18]);
        let output = cpu.run();

        assert_eq!(&[3, 3, 1107, 0, 8, 3, 4, 3, 99], cpu.mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_5() {
        // Input and results taken from example
        let mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut cpu = Registers::new(&mem, &[0]);
        let output = cpu.run();
        assert_eq!(&[0], output.as_slice());

        let mut cpu = Registers::new(&mem, &[50]);
        let output = cpu.run();
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_6() {
        // Input and results taken from example
        let mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let output = Registers::new(&mem, &[0]).run();
        assert_eq!(&[0], output.as_slice());

        let output = Registers::new(&mem, &[50]).run();
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_7() {
        // Input and results taken from example
        let mem = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let output = Registers::new(&mem, &[0]).run();
        assert_eq!(&[999], output.as_slice());

        let output = Registers::new(&mem, &[8]).run();
        assert_eq!(&[1000], output.as_slice());

        let output = Registers::new(&mem, &[878329]).run();
        assert_eq!(&[1001], output.as_slice());
    }

    #[test]
    fn examle_day9_1() {
        // Takes no input and produces a copy of itself as output.
        let mem = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut cpu = Registers::new(&mem, &[]);
        let result = cpu.run();

        println!("{:?}", cpu.mem);
        assert_eq!(result, mem);
    }

    #[test]
    fn examle_day9_2() {
        // outputs 16 digit number
        let mem = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let mut cpu = Registers::new(&mem, &[]);
        let result = cpu.run();

        assert_eq!(&[1219070632396864], result.as_slice());
    }

    #[test]
    fn examle_day9_3() {
        // should output the large number in the middle
        let mem = [104, 1125899906842624, 99];

        let mut cpu = Registers::new(&mem, &[]);
        let result = cpu.run();

        assert_eq!(&[1125899906842624], result.as_slice());
    }
}
