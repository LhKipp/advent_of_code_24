use std::collections::HashMap;

enum ResultOrSingleDigitNumber {
    Result(usize),
    SingleDigit { digit: usize, i: usize },
}

struct SingleDigit {
    digit: usize,
    i: usize,
}

// fn calculate_rule_lengths_per_blink() -> HashMap<usize, Vec<(usize, Vec<SingleDigit>)>> {
//     let mut result = HashMap::<usize, Vec<(usize, Vec<SingleDigit>)>>::new();
//
//     for i in 0..10 {
//         result.insert(i, vec![]);
//     }
//
//     for i in 0..10 {
//         println!("Calculating blinks of {}", i);
//
//         let result_vec = result.get_mut(&i).unwrap();
//         let mut cur = vec![i];
//         let mut tmp = vec![];
//         for _ in 0..75 {
//             tmp.clear();
//             for v in &cur {
//                 if *v == 0 {
//                     tmp.push(*v);
//                 } else {
//                     let v_str = v.to_string();
//                     if v_str.len() % 2 == 0 {
//                         let (a, b) = v_str.split_at(v_str.len() / 2);
//                         tmp.push(a.parse().unwrap());
//                         tmp.push(b.parse().unwrap());
//                     } else {
//                         tmp.push(v * 2024);
//                     }
//                 }
//             }
//             std::mem::swap(&mut cur, &mut tmp);
//
//             for
//         }
//     }
//
//     result
// }

fn insert_or_add(v: usize, count: usize, m: &mut HashMap<usize, usize>) {
    match m.entry(v) {
        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
            *occupied_entry.get_mut() += count;
        }
        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(count);
        }
    }
}

fn blink_75_times(init: &[usize]) -> usize {
    let mut numbers_and_count = HashMap::<usize, usize>::new();

    for v in init {
        insert_or_add(*v, 1, &mut numbers_and_count);
    }

    let mut tmp = HashMap::<usize, usize>::new();
    for _ in 0..75 {
        tmp.clear();
        for (number, count) in &numbers_and_count {
            if *number == 0 {
                insert_or_add(1, *count, &mut tmp);
            } else {
                let k_str = number.to_string();
                if k_str.len() % 2 == 0 {
                    let (a, b) = k_str.split_at(k_str.len() / 2);
                    insert_or_add(a.parse().unwrap(), *count, &mut tmp);
                    insert_or_add(b.parse().unwrap(), *count, &mut tmp);
                } else {
                    insert_or_add(*number * 2024, *count, &mut tmp);
                }
            }
        }
        std::mem::swap(&mut numbers_and_count, &mut tmp);
        println!(
            "one iteration done. size {}, {:?}",
            numbers_and_count.values().sum::<usize>(),
            numbers_and_count
        );
    }
    numbers_and_count.values().sum()
}

fn main() {
    let input = [3028, 78, 973951, 5146801, 5, 0, 23533, 857];
    // let example = vec![125, 17];
    println!("{}", blink_75_times(&input));
}
