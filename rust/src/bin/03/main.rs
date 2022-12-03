use std::collections::HashSet;

fn main() {
    let backpacks = read_backpacks(include_str!("../../../../inputs/03.txt"));
    let num_backpacks = backpacks.len();
    let mut individual_duplicate_sum: i32 = 0;
    for (left, right) in backpacks {
        let duplicate = find_duplicate_item(left, right);
        individual_duplicate_sum += item_to_priority(duplicate) as i32;
    }
    println!("individual duplicate sum: {individual_duplicate_sum}");

    let mut group_badge_sum: i32 = 0;
    let mut lines = include_str!("../../../../inputs/03.txt").lines();
    for _ in 0..num_backpacks / 3 {
        let group_badge = find_group_badge(
            lines.next().expect("no line left"),
            lines.next().expect("no line left"),
            lines.next().expect("no line left"),
        );
        group_badge_sum += item_to_priority(group_badge) as i32;
    }
    println!("group badge sum: {group_badge_sum}");
}

fn read_backpacks(input: &str) -> Vec<(&str, &str)> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.split_at(line.len() / 2));
    }
    result
}

fn item_to_priority(item: char) -> i32 {
    match item.is_lowercase() {
        true => (item as i32) - ('a' as i32) + 1,
        false => (item as i32) - ('A' as i32) + 1 + 26,
    }
}

fn find_duplicate_item(compartment1: &str, compartment2: &str) -> char {
    let chars1: HashSet<char> = HashSet::from_iter(compartment1.chars());
    let chars2: HashSet<char> = HashSet::from_iter(compartment2.chars());
    *chars1
        .intersection(&chars2)
        .next()
        .expect("No intersection")
}

fn find_group_badge(backpack1: &str, backpack2: &str, backpack3: &str) -> char {
    let chars1: HashSet<char> = HashSet::from_iter(backpack1.chars());
    let chars2: HashSet<char> = HashSet::from_iter(backpack2.chars());
    let chars3: HashSet<char> = HashSet::from_iter(backpack3.chars());

    // there must be a better way to intersect three sets...
    let intersection12: HashSet<char> =
        HashSet::from_iter(chars1.intersection(&chars2).into_iter().map(char::clone));
    *chars3
        .intersection(&intersection12)
        .next()
        .expect("No intersection")
}

#[cfg(test)]
mod tests {
    use crate::{find_duplicate_item, find_group_badge, item_to_priority, read_backpacks};

    #[test]
    fn test_read_backpacks_on_empty_input() {
        assert_eq!(read_backpacks(""), Vec::new());
    }

    #[test]
    fn test_read_backpacks() {
        let input_string = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .join("\n");
        let backpacks = read_backpacks(input_string.as_str());

        assert_eq!(
            backpacks,
            vec![
                ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
                ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
                ("PmmdzqPrV", "vPwwTWBwg"),
                ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
                ("ttgJtRGJ", "QctTZtZT"),
                ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
            ]
        )
    }

    #[test]
    fn test_item_to_priority() {
        assert_eq!(item_to_priority('a'), 1);
        assert_eq!(item_to_priority('z'), 26);
        assert_eq!(item_to_priority('A'), 27);
        assert_eq!(item_to_priority('Z'), 52);

        assert_eq!(item_to_priority('p'), 16);
        assert_eq!(item_to_priority('L'), 38);
        assert_eq!(item_to_priority('P'), 42);
        assert_eq!(item_to_priority('v'), 22);
        assert_eq!(item_to_priority('t'), 20);
        assert_eq!(item_to_priority('s'), 19);
    }

    #[test]
    fn test_find_duplicate_item() {
        assert_eq!(find_duplicate_item("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
        assert_eq!(
            find_duplicate_item("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            'L'
        );
        assert_eq!(find_duplicate_item("PmmdzqPrV", "vPwwTWBwg"), 'P');
        assert_eq!(
            find_duplicate_item("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            'v'
        );
        assert_eq!(find_duplicate_item("ttgJtRGJ", "QctTZtZT"), 't');
        assert_eq!(find_duplicate_item("CrZsJsPPZsGz", "wwsLwLmpwMDw"), 's');
    }

    #[test]
    fn test_find_group_badge() {
        assert_eq!(
            find_group_badge(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            'r'
        );
        assert_eq!(
            find_group_badge(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            'Z'
        );
    }
}
