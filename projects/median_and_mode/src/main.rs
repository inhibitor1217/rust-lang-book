fn main() {
    assert_eq!(median(&vec![3, 1, 4, 1, 5]), Some(3));
    assert_eq!(median(&vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]), Some(3));
    assert_eq!(median(&vec![]), None);

    assert_eq!(mode(&vec![3, 1, 4, 1, 5]), Some(1));
    assert_eq!(mode(&vec![3, 3, 3, 4, 5, 5, 5, 5]), Some(5));
    assert_eq!(mode(&vec![]), None);
}

fn median(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        Option::None
    } else {
        let median_index = (v.len() - 1) / 2;
        let mut sorted = v.clone();
        sorted.sort_unstable();
        sorted.get(median_index).map(|&i| i)
    }
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        Option::None
    } else {
        use std::collections::HashMap;
        let mut counts: HashMap<i32, u32> = HashMap::new();
        for &num in v {
            let entry = counts.entry(num).or_insert(0);
            *entry += 1;
        }

        counts
            .into_iter()
            .max_by(|left, right| left.1.cmp(&right.1))
            .map(|x| x.0)
    }
}
