use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let ore_for_1 = calc_ore_need(1);
    println!("PART1: You need {} ore", ore_for_1);

    let input_ore: usize = 1_000_000_000_000;

    let mut lower_bound = input_ore / ore_for_1;
    let mut upper_bound = input_ore / 100_000;

    let part2_fuel;
    loop {
        if upper_bound - lower_bound == 1 {
            part2_fuel = lower_bound;
            break;
        }

        let center = lower_bound + (upper_bound - lower_bound) / 2;
        let ore_need = calc_ore_need(center);
        println!(
            "L{} C{}({}) H{}",
            lower_bound, center, ore_need, upper_bound
        );
        if ore_need == input_ore {
            part2_fuel = ore_need;
            break;
        } else if ore_need > input_ore {
            upper_bound = center;
        } else if ore_need < input_ore {
            lower_bound = center
        }
    }

    println!("PART2: {}", part2_fuel);
}

fn calc_ore_need(fuel: usize) -> usize {
    let input = include_str!("input.txt");
    let mut reactions = parse_reactions(input);

    let mut ore: usize = 0;

    let mut eval_que: VecDeque<(String, usize)> = VecDeque::new();
    eval_que.push_back(("FUEL".to_string(), fuel));

    while let Some((element, amount)) = eval_que.pop_front() {
        if element == "ORE" {
            ore += amount as usize;
            continue;
        }

        let reaction_info = reactions.get_mut(&element).unwrap();
        reaction_info.supply -= amount as isize;

        // Produce extra if needed
        if reaction_info.supply < 0 {
            let need = reaction_info.supply.abs();
            let nr_of_reactions = (need as f64 / reaction_info.output as f64).ceil() as usize;

            for (in_elem, in_amount) in reaction_info.input.iter() {
                eval_que.push_back((in_elem.to_string(), in_amount * nr_of_reactions));
            }
            reaction_info.supply += (reaction_info.output * nr_of_reactions) as isize;
        }
    }

    ore
}

#[derive(Debug)]
struct ReactionInfo {
    output: usize,
    supply: isize,
    input: Vec<(String, usize)>,
}

fn parse_reactions(input: &str) -> HashMap<String, ReactionInfo> {
    input.lines().map(parse_reaction).collect()
}

// 1 PTGXM, 13 WMJW, 1 BJGD => 7 KDHF
fn parse_reaction(line: &str) -> (String, ReactionInfo) {
    let mut parts = line.split(" => ");
    let inputs = parts.next().unwrap().split(", ");
    let inputs = inputs.map(parse_element).collect();
    let (element, nr) = parse_element(parts.next().unwrap());

    (
        element,
        ReactionInfo {
            output: nr,
            supply: 0,
            input: inputs,
        },
    )
}

fn parse_element(input: &str) -> (String, usize) {
    let mut split = input.split(" ");
    let nr_elements = split.next().unwrap();
    let element = split.next().unwrap();
    (element.to_string(), nr_elements.parse().unwrap())
}
