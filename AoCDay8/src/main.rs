fn main() {
    let input = include_str!("input.txt");

    let input: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).expect("Only digits expected") as u8)
        .collect();

    // 25 x 6 pixels
    // Layer with the fewest 0 digits
    let (layer_id, _nr_zeros) = input
        .chunks_exact(25 * 6)
        .enumerate()
        .map(|(i, chunk)| (i, count_zeros(chunk)))
        .fold((0, std::usize::MAX), |(a, min_zeros), (i, nr_zeros)| {
            if nr_zeros < min_zeros {
                (i, nr_zeros)
            } else {
                (a, min_zeros)
            }
        });

    // number of 1 digits multiplied by the number of 2 digits of the layer with the fewest 0
    let layer: &[u8] = input
        .chunks_exact(25 * 6)
        .nth(layer_id)
        .expect("Layer is there");
    let nr_of_1_digits = layer.iter().filter(|&&i| i == 1).count();
    let nr_of_2_digits = layer.iter().filter(|&&i| i == 2).count();

    println!("PART1: {}", nr_of_1_digits * nr_of_2_digits);

    // 0 is black, 1 is white, and 2 is transparent.
    let image = fold_layers(&input, 25 * 6);

    let str_image = image
        .chunks_exact(25)
        .map(|line| {
            //invert colors to make it readable on a white background
            line.into_iter()
                .map(|&c| if c == 0 { ' ' } else { '█' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    print!("PART2: \n{}", str_image);
}

fn count_zeros(input: &[u8]) -> usize {
    input.iter().filter(|&&v| v == 0).count()
}

fn fold_layers(input: &[u8], image_size: usize) -> Vec<u8> {
    // 0 is black, 1 is white, and 2 is transparent.
    input
        .chunks_exact(image_size)
        .fold(vec![2; image_size], |image, layer| {
            image
                .into_iter()
                .zip(layer.into_iter())
                .map(|(top, &bottom)| if top == 2 { bottom } else { top })
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part2_1() {
        let input = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        assert_eq!(&fold_layers(&input, 2 * 2), &[0, 1, 1, 0]);
    }
}
