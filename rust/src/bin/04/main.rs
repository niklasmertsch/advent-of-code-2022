use std::collections::HashSet;

fn main() {
    let mut full_overlaps = 0;
    let mut partial_overlaps = 0;
    let mut first_set: HashSet<i32> = HashSet::new();
    let mut second_set: HashSet<i32> = HashSet::new();
    for line in include_str!("../../../../inputs/04.txt").lines() {
        let mut s = line
            .trim()
            .split(',')
            .flat_map(|s| s.split('-'))
            .map(|s| s.parse::<i32>().expect("no int"));

        let first_start = s.next().expect("no first");
        let first_end = s.next().expect("no second");
        let second_start = s.next().expect("no third");
        let second_end = s.next().expect("no fourth");

        first_set.clear();
        first_set.extend(first_start..first_end + 1);
        second_set.clear();
        second_set.extend(second_start..second_end + 1);

        if first_set.is_subset(&second_set) || first_set.is_superset(&second_set) {
            full_overlaps += 1;
        }
        if !first_set.is_disjoint(&second_set) {
            partial_overlaps += 1;
        }
    }

    println!("{}", full_overlaps);
    println!("{}", partial_overlaps);
}
