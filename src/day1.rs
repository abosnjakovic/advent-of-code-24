/// # Day 1 answer 1
pub fn answer(input: &str) -> u32 {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    let mut count = 0;

    // Process each line and split on 3 spaces
    for line in input.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            list1.push(first.trim().parse().unwrap());
            list2.push(second.trim().parse().unwrap());
        }
    }

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        if list1[i] >= list2[i] {
            count += list1[i] - list2[i];
        } else {
            count += list2[i] - list1[i];
        }
    }

    count
}

pub fn answer2(input: &str) -> u32 {
    0
}
