const DAY: &str = "02";

pub fn prepare_input(input: &String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|string| string.parse::<usize>().expect("Cannot parse day 2"))
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn part_one(input: &String) -> Option<usize> {
    let prepared_input = prepare_input(input);

    let total_wrapping = prepared_input
        .iter()
        .map(|array| match array[..3] {
            [l, w, h] => {
                2 * l * w + 2 * w * h + 2 * h * l + [l * h, l * w, w * h].into_iter().min().unwrap()
            }
            _ => panic!("not supposed to happend"),
        })
        .sum();

    Some(total_wrapping)
}

pub fn part_two(input: &String) -> Option<usize> {
    let prepared_input = prepare_input(input);

    let total_wrapping = prepared_input
        .iter()
        .map(|array| match array[..3] {
            [l, w, h] => {
                l * h * w
                    + [2 * (l + h), 2 * (l + w), 2 * (w + h)]
                        .into_iter()
                        .min()
                        .unwrap()
            }
            _ => panic!("not supposed to happend"),
        })
        .sum();

    Some(total_wrapping)
}

#[cfg(test)]
mod tests_day2 {
    use super::*;
    use crate::puzzles;

    #[test]
    fn test_part_one() {
        let result = part_one(&puzzles::read_input(DAY));
        assert_eq!(Some(1606483), result)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&puzzles::read_input(DAY));
        assert_eq!(Some(3842356), result)
    }
}
