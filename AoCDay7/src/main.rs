extern crate permutohedron;
use permutohedron::LexicalPermutation;

use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    // Input program
    let mem = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 30, 51, 72, 81, 94, 175, 256, 337, 418, 99999, 3,
        9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 3, 9, 1002, 9, 2, 9, 1001, 9, 2, 9, 1002, 9, 5,
        9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 101, 4, 9, 9, 102, 5, 9, 9, 101, 3, 9, 9, 4, 9, 99, 3, 9,
        1002, 9, 4, 9, 4, 9, 99, 3, 9, 102, 3, 9, 9, 1001, 9, 4, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9,
        4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2,
        9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
        3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99,
    ];

    let mut max_trust = 0;
    let mut max_phase_setting = [0; 5];

    let mut phase_setting = [0, 1, 2, 3, 4];
    loop {
        let trust = calc_trust(&phase_setting, &mem);
        if trust > max_trust {
            max_trust = trust;
            max_phase_setting = phase_setting;
        }
        if !phase_setting.next_permutation() {
            break;
        }
    }

    println!(
        "PART 1: trust signal: {} phase settings: {:?}",
        max_trust, max_phase_setting
    );

    let mut max_trust = 0;
    let mut max_phase_setting = [0; 5];

    let mut phase_setting = [5, 6, 7, 8, 9];
    loop {
        let trust = calc_trust_part2(&phase_setting, &mem);
        if trust > max_trust {
            max_trust = trust;
            max_phase_setting = phase_setting;
        }
        if !phase_setting.next_permutation() {
            break;
        }
    }

    println!(
        "PART 2: trust signal: {} phase settings: {:?}",
        max_trust, max_phase_setting
    );
}

fn calc_trust(phase_setting: &[isize], mem: &[isize]) -> isize {
    let mut carry = 0;
    for &phase in phase_setting {
        carry = Registers::new(&mem, &[phase, carry]).run()[0];
    }
    carry
}

fn calc_trust_part2(phase_setting: &[isize], mem: &[isize]) -> isize {
    let mut cpus: Vec<Registers> = phase_setting
        .into_iter()
        .map(|&phase| Registers::new(mem, &[phase]))
        .collect();

    let mut carry = 0;
    let mut result = 0;
    for i in (0..phase_setting.len()).into_iter().cycle() {
        cpus[i].add_input(carry);
        if let Some(output) = cpus[i].step() {
            carry = output;
            if i == phase_setting.len() - 1 {
                result = output;
            }
        } else {
            break;
        }
    }

    result
}

struct Registers {
    mem: Vec<isize>,
    pc: usize,
    mode: [u8; 3],
    opcode: u8,
    input: VecDeque<isize>,
}

impl Registers {
    fn new(mem: &[isize], input: &[isize]) -> Registers {
        let mut reg = Registers {
            mem: mem.to_vec(),
            pc: 0,
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
            0 => self.mem[self.mem[self.pc + i] as usize], // Position / pointer mode
            1 => self.mem[self.pc + i],                    // Paramter mode
            _ => panic!("Unsupported mode"),
        }
    }

    fn write(&mut self, arg: usize, value: isize) {
        let location = self.mem[self.pc + arg] as usize;
        self.mem[location] = value; // Position / pointer mode
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
    fn examle_day7_1() {
        let mem = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(43210, calc_trust(&[4, 3, 2, 1, 0], &mem));
    }

    #[test]
    fn examle_day7_2() {
        let mem = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(54321, calc_trust(&[0, 1, 2, 3, 4], &mem));
    }

    #[test]
    fn examle_day7_3() {
        let mem = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(65210, calc_trust(&[1, 0, 4, 3, 2], &mem));
    }
}
