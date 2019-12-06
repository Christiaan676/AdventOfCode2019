fn main() {
    // Input program
    let mut init_mem = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 2, 19, 13, 23, 1, 23, 10, 27,
        1, 13, 27, 31, 2, 31, 10, 35, 1, 35, 9, 39, 1, 39, 13, 43, 1, 13, 43, 47, 1, 47, 13, 51, 1,
        13, 51, 55, 1, 5, 55, 59, 2, 10, 59, 63, 1, 9, 63, 67, 1, 6, 67, 71, 2, 71, 13, 75, 2, 75,
        13, 79, 1, 79, 9, 83, 2, 83, 10, 87, 1, 9, 87, 91, 1, 6, 91, 95, 1, 95, 10, 99, 1, 99, 13,
        103, 1, 13, 103, 107, 2, 13, 107, 111, 1, 111, 9, 115, 2, 115, 10, 119, 1, 119, 5, 123, 1,
        123, 2, 127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
    ];

    // make sure last opcode 99 has 4 values
    init_mem.push(0);
    init_mem.push(0);
    init_mem.push(0);

    init_mem[1] = 12;
    init_mem[2] = 2;

    let mut mem = init_mem.clone();
    intcode_processor(&mut mem);
    println!("The value at 0: {}", mem[0]);

    // PART 2
    'noun: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = init_mem.clone();
            mem[1] = noun;
            mem[2] = verb;
            intcode_processor(&mut mem);

            if mem[0] == 19690720 {
                println!(
                    "noun: {} verb: {} answer: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                break 'noun;
            }
        }
    }
}

fn intcode_processor(input: &mut [usize]) {
    for i in 0.. {
        let index = i * 4;
        match input[index..index + 4] {
            [1, a, b, r] => {
                // add
                input[r] = input[a] + input[b];
            }
            [2, a, b, r] => {
                // mull
                input[r] = input[a] * input[b];
            }
            [99, _, _, _] => {
                // Halt
                return;
            }
            _ => {
                // Somethings wronge
                return;
            }
        }
    }
}
