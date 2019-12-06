fn main() {
    // Input program
    let mem = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 91, 92, 225, 1102, 85, 13, 225, 1,
        47, 17, 224, 101, -176, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 223,
        224, 223, 1102, 79, 43, 225, 1102, 91, 79, 225, 1101, 94, 61, 225, 1002, 99, 42, 224, 1001,
        224, -1890, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 102, 77,
        52, 224, 1001, 224, -4697, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 224, 223,
        223, 1101, 45, 47, 225, 1001, 43, 93, 224, 1001, 224, -172, 224, 4, 224, 102, 8, 223, 223,
        1001, 224, 1, 224, 1, 224, 223, 223, 1102, 53, 88, 225, 1101, 64, 75, 225, 2, 14, 129, 224,
        101, -5888, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 101,
        60, 126, 224, 101, -148, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2, 224, 1, 224,
        223, 223, 1102, 82, 56, 224, 1001, 224, -4592, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224,
        224, 1, 224, 223, 223, 1101, 22, 82, 224, 1001, 224, -104, 224, 4, 224, 1002, 223, 8, 223,
        101, 4, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 1001, 223, 1, 223,
        1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 108, 226, 226,
        224, 1002, 223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223,
        223, 1006, 224, 374, 101, 1, 223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 389,
        1001, 223, 1, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 101, 1, 223,
        223, 7, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 1108, 226, 677,
        224, 1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223,
        223, 1005, 224, 449, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1005, 224,
        464, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223,
        223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1008, 226,
        226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 1108, 677, 226, 224, 1002,
        223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2, 223,
        1005, 224, 539, 101, 1, 223, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554,
        101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1,
        223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 584, 1001, 223, 1, 223, 7, 677, 226,
        224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2,
        223, 1005, 224, 614, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
        629, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 644, 1001, 223,
        1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 659, 1001, 223, 1, 223, 107, 677,
        677, 224, 1002, 223, 2, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ];

    let output = intcode_processor(&mut mem.clone(), &[1]);
    println!("PART 1: The value at 0: {} output: {:?}", mem[0], output);

    let output = intcode_processor(&mut mem.clone(), &[5]);
    println!("PART 2: The value at 0: {} output: {:?}", mem[0], output);
}

struct Registers<'a> {
    mem: &'a mut [isize],
    pc: usize,
    mode: [u8; 3],
    opcode: u8,
}

impl<'a> Registers<'a> {
    fn new(mem: &mut [isize]) -> Registers {
        let mut reg = Registers {
            mem,
            pc: 0,
            mode: [0; 3],
            opcode: 0,
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
            0 => self.mem[self.mem[self.pc + i] as usize], // Position / pointer mode
            1 => self.mem[self.pc + i],                    // Paramter mode
            _ => panic!("Unsupported mode"),
        }
    }

    fn write(&mut self, arg: usize, value: isize) {
        self.mem[self.mem[self.pc + arg] as usize] = value; // Position / pointer mode
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
}

fn intcode_processor(mem: &mut [isize], input: &[isize]) -> Vec<isize> {
    let mut input_it = input.into_iter();
    let mut output = vec![];

    let mut reg = Registers::new(mem);

    loop {
        match reg.get_opcode() {
            1 => {
                // add
                reg.write(3, reg.get_arg(1) + reg.get_arg(2));
                reg.increment_pc(4);
            }
            2 => {
                // mull
                reg.write(3, reg.get_arg(1) * reg.get_arg(2));
                reg.increment_pc(4);
            }
            3 => {
                // Input
                reg.write(1, *input_it.next().unwrap_or(&0));
                reg.increment_pc(2);
            }
            4 => {
                // Ouput
                output.push(reg.get_arg(1));
                reg.increment_pc(2);
            }
            5 => {
                //jump-if-true
                if reg.get_arg(1) != 0 {
                    reg.goto(reg.get_arg(2) as usize);
                } else {
                    reg.increment_pc(3);
                }
            }
            6 => {
                //jump-if-false
                if reg.get_arg(1) == 0 {
                    reg.goto(reg.get_arg(2) as usize);
                } else {
                    reg.increment_pc(3);
                }
            }
            7 => {
                //less than
                if reg.get_arg(1) < reg.get_arg(2) {
                    reg.write(3, 1);
                } else {
                    reg.write(3, 0);
                }
                reg.increment_pc(4);
            }
            8 => {
                //equals
                if reg.get_arg(1) == reg.get_arg(2) {
                    reg.write(3, 1);
                } else {
                    reg.write(3, 0);
                }
                reg.increment_pc(4);
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

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examle_day2_1() {
        // Input and results taken from example
        let mut mem = vec![1, 0, 0, 0, 99];
        let _output = intcode_processor(&mut mem, &[]);

        assert_eq!(&[2, 0, 0, 0, 99], mem.as_slice());
    }

    #[test]
    fn examle_day2_2() {
        // Input and results taken from example
        let mut mem = vec![2, 3, 0, 3, 99];
        let _output = intcode_processor(&mut mem, &[]);

        assert_eq!(&[2, 3, 0, 6, 99], mem.as_slice());
    }

    #[test]
    fn examle_day2_3() {
        // Input and results taken from example
        let mut mem = vec![2, 4, 4, 5, 99, 0];
        let _output = intcode_processor(&mut mem, &[]);

        assert_eq!(&[2, 4, 4, 5, 99, 9801], mem.as_slice());
    }

    #[test]
    fn examle_day2_4() {
        // Input and results taken from example
        let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let _output = intcode_processor(&mut mem, &[]);

        assert_eq!(&[30, 1, 1, 4, 2, 5, 6, 0, 99], mem.as_slice());
    }

    #[test]
    fn examle_day5_1() {
        // Input and results taken from example
        let mut mem = vec![3, 0, 4, 0, 99];
        let output = intcode_processor(&mut mem, &[5678]);

        assert_eq!(&[5678, 0, 4, 0, 99], mem.as_slice());
        assert_eq!(&[5678], output.as_slice());
    }

    #[test]
    fn examle_day5_2() {
        // Input and results taken from example
        let mut mem = vec![1101, 100, -1, 4, 0];
        let output = intcode_processor(&mut mem, &[5678]);

        assert_eq!(&[1101, 100, -1, 4, 99], mem.as_slice());
        assert!(output.is_empty());
    }

    #[test]
    fn examle_day5_part2_1a() {
        // Input and results taken from example
        let mut mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode_processor(&mut mem, &[8]);

        assert_eq!(&[3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_1b() {
        // Input and results taken from example
        let mut mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode_processor(&mut mem, &[18]);

        assert_eq!(&[3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8], mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_2a() {
        // Input and results taken from example
        let mut mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode_processor(&mut mem, &[5]);

        assert_eq!(&[3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8], mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_2b() {
        // Input and results taken from example
        let mut mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode_processor(&mut mem, &[18]);

        assert_eq!(&[3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_3a() {
        // Input and results taken from example
        let mut mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output = intcode_processor(&mut mem, &[8]);

        assert_eq!(&[3, 3, 1108, 1, 8, 3, 4, 3, 99], mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_3b() {
        // Input and results taken from example
        let mut mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output = intcode_processor(&mut mem, &[18]);

        assert_eq!(&[3, 3, 1108, 0, 8, 3, 4, 3, 99], mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_4a() {
        // Input and results taken from example
        let mut mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output = intcode_processor(&mut mem, &[5]);

        assert_eq!(&[3, 3, 1107, 1, 8, 3, 4, 3, 99], mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_4b() {
        // Input and results taken from example
        let mut mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output = intcode_processor(&mut mem, &[18]);

        assert_eq!(&[3, 3, 1107, 0, 8, 3, 4, 3, 99], mem.as_slice());
        assert_eq!(&[0], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_5() {
        // Input and results taken from example
        let mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let output = intcode_processor(&mut mem.clone(), &[0]);
        //assert_eq!(&[3,3,1107,0,8,3,4,3,99], mem.as_slice());
        assert_eq!(&[0], output.as_slice());

        let output = intcode_processor(&mut mem.clone(), &[50]);
        //assert_eq!(&[3,3,1107,0,8,3,4,3,99], mem.as_slice());
        assert_eq!(&[1], output.as_slice());
    }

    #[test]
    fn examle_day5_part2_6() {
        // Input and results taken from example
        let mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let output = intcode_processor(&mut mem.clone(), &[0]);
        assert_eq!(&[0], output.as_slice());

        let output = intcode_processor(&mut mem.clone(), &[50]);
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

        let output = intcode_processor(&mut mem.clone(), &[0]);
        assert_eq!(&[999], output.as_slice());

        let output = intcode_processor(&mut mem.clone(), &[8]);
        assert_eq!(&[1000], output.as_slice());

        let output = intcode_processor(&mut mem.clone(), &[878329]);
        assert_eq!(&[1001], output.as_slice());
    }
}
