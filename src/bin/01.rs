pub fn part_one(input: &str) -> Option<u32> {
    let vec: Vec<u32> = convert(input);
    Some(vec.iter().cloned().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec: Vec<u32> = convert(input);
    vec.sort();
    vec.reverse();
    Some(vec[0] + vec[1] + vec[2])
}

fn convert(input: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    let mut total = 0;
    for line in input.lines() {
        if line == "" {
            vec.push(total);
            total = 0;
        } else {
            total += line.parse::<u32>().unwrap();
        }
    }
    vec
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
