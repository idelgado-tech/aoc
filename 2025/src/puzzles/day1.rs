const DAY: &str = "01";

fn tourne_et_valide_si_zero_a_la_fin(acc: &mut usize) -> impl '_ + FnOnce(&'_ str) -> usize {
    move |line| {
        let (sens, value) = line.split_at(1);
        let parsed_value = value.parse::<usize>().unwrap() % 100;
        match sens {
            "R" => {
                *acc = (*acc + parsed_value) % 100;
            }
            "L" => {
                let temp: isize = *acc as isize - parsed_value as isize;
                println!("here {}", temp);

                if temp < 0 {
                    *acc = 100 - temp.abs() as usize;
                } else {
                    *acc = temp as usize
                }
            }
            _ => panic!("Ne devrais pas se produire"),
        }

        if *acc == 0 { return 1 } else { return 0 }
    }
}

pub fn part_one(input: &String) -> usize {
    let mut acc: usize = 50;
    let lines = input.lines();

    lines
        .into_iter()
        .map(|line| tourne_et_valide_si_zero_a_la_fin(&mut acc)(line))
        .sum::<usize>()
}

fn tourne_et_valide_si_zero(acc: &mut usize) -> impl '_ + FnOnce(&'_ str) -> usize {
    move |line| {
        let (sens, value) = line.split_at(1);
        let parsed_value = value.parse::<usize>().unwrap() % 100;
        let tours = value.parse::<usize>().unwrap().div_euclid(100);
        let valeur_de_depart = *acc;
        let mut passed_over = 0;

        match sens {
            "R" => {
                *acc = (*acc + parsed_value) % 100;
                if *acc < valeur_de_depart && valeur_de_depart != 0 && *acc != 0 {
                    passed_over = 1
                };
            }
            "L" => {
                let temp: isize = *acc as isize - parsed_value as isize;

                if temp < 0 {
                    *acc = 100 - temp.abs() as usize;
                } else {
                    *acc = temp as usize
                }

                if *acc > valeur_de_depart && valeur_de_depart != 0 && *acc != 0 {
                    passed_over = 1
                };
            }
            _ => panic!("Ne devrais pas se produire"),
        }

        let is_in_zero = if *acc == 0 { 1 } else { 0 };

        println!(
            "start at {} arrive at {} after {} , passed over ? {} ,is in zero {} total {}",
            valeur_de_depart,
            *acc,
            line,
            passed_over,
            is_in_zero,
            tours + passed_over + is_in_zero
        );

        return tours + passed_over + is_in_zero;
    }
}

pub fn part_two(input: &String) -> usize {
    let mut acc: usize = 50;
    let lines = input.lines();

    lines
        .into_iter()
        .map(|line| tourne_et_valide_si_zero(&mut acc)(line))
        .sum::<usize>()
}

#[cfg(test)]
mod tests_day1 {
    use super::*;
    use crate::puzzles;

    #[test]
    fn test_exemple_part_one() {
        let result = part_one(&puzzles::read_exemple(DAY));
        assert_eq!(3, result)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&puzzles::read_input(DAY));
        assert_eq!(1059, result)
    }

    #[test]
    fn test_exemple_part_two() {
        let result = part_two(&puzzles::read_exemple(DAY));
        assert_eq!(6, result)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&puzzles::read_input(DAY));
        assert_eq!(6305, result)
    }
}
