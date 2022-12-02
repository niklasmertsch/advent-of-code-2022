fn main() {
    let input = include_str!("input.txt");
    let mut calories = get_calories(input);
    calories.sort();
    let max_calories = calories[calories.len() - 1];
    let sum_max_3_calories: i32 = calories[calories.len() - 3 .. calories.len()].iter().sum();

    println!("max: {}", max_calories);
    println!("max 3: {}", sum_max_3_calories);
}

fn get_calories(input: &str) -> Vec<i32> {
    let mut calories = Vec::new();
    let mut current_kcal = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(kcal) => { current_kcal += kcal }
            Err(_) => {
                calories.push(current_kcal);
                current_kcal = 0;
            }
        }
    }
    if current_kcal > 0 {
        calories.push(current_kcal);
    }
    return calories;
}

#[cfg(test)]
mod tests {
    use crate::{get_calories};

    #[test]
    fn test_get_calories_on_empty_input() {
        let calories = get_calories("");

        assert_eq!(calories.len(), 0);
    }

    #[test]
    fn test_get_calories() {
        let calories = get_calories("1\n2\n123\n\n12\n\n13\n51\n");

        assert_eq!(calories.len(), 3);

        assert_eq!(calories[0], 126);
        assert_eq!(calories[1], 12);
        assert_eq!(calories[2], 64);
    }
}
