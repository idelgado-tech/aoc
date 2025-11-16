const DAY: &str = "01";

pub fn part_one(input: &String) -> Option<i64> {
    let mut acc = 0;
    for char in input.chars() {
        match char {
            '(' => acc += 1,
            ')' => acc -= 1,
            _ => panic!("Not supposed to happend"),
        }
    }

    Some(acc)
}

pub fn part_two(input: &String) -> Option<usize> {
    let mut acc = 0;
    let mut enter_basement = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => acc += 1,
            ')' => acc -= 1,
            _ => panic!("Not supposed to happend"),
        }
        if acc < 0 && enter_basement == 0 {
            enter_basement = index + 1;
        }
    }

    Some(enter_basement)
}

#[cfg(test)]
mod tests_day1 {
    use super::*;
    use crate::puzzles;

    #[test]
    fn test_part_one() {
        let result = part_one(&puzzles::read_input(DAY));
        assert_eq!(Some(280), result)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&puzzles::read_input(DAY));
        assert_eq!(Some(1797), result)
    }
}
