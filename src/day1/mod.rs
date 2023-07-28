fn max_calory(input: &str) -> u32 {
    let mut max = 0;

    let mut current = 0;

    for line in input.lines() {
        if let Ok(value) = line.parse::<u32>() {
            current += value;
        } else {
            if current > max {
                max = current;
            }
            current = 0;
        }
    }

    max
}

fn top3_calories(input: &str) -> u32 {
    let mut top3 = [0; 3];

    let mut current = 0;

    for line in input.lines().chain(std::iter::once("\n")) {
        if let Ok(value) = line.parse::<u32>() {
            current += value;
        } else {
            if current > top3[2] {
                top3[0] = top3[1];
                top3[1] = top3[2];
                top3[2] = current;
            } else if current > top3[1] {
                top3[0] = top3[1];
                top3[1] = current;
            } else if current > top3[0] {
                top3[0] = current;
            }
            current = 0;
        }
    }

    top3.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_example_max_calories() {
        assert_eq!(max_calory(SAMPLE_INPUT), 24000);
    }

    #[test]
    fn test_input_max_calories() {
        println!("Max calorie: {:?}", max_calory(INPUT));
    }

    #[test]
    fn test_example_top3_calories() {
        assert_eq!(top3_calories(SAMPLE_INPUT), 24000 + 11000 + 10000);
    }

    #[test]
    fn test_input_top3_calories() {
        println!("Sum of Top3 calories: {:?}", top3_calories(INPUT));
    }
}
