use core::fmt;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash)]
struct Rule {
    before: i32,
    after: i32,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}", self.before, self.after)
    }
}

fn parse_rules_from_file(filepath: &str) -> io::Result<HashSet<Rule>> {
    let mut rules_set = HashSet::new();

    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split('|').collect();
        assert!(parts.len() == 2);
        let (before, after) = (
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        );
        rules_set.insert(Rule { before, after });
    }

    Ok(rules_set)
}

fn parse_updates_from_file(filepath: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut result = Vec::new();

    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let integers: Vec<i32> = line
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        result.push(integers);
    }

    Ok(result)
}

fn is_valid(rules: &HashSet<Rule>, update: &[i32]) -> bool {
    // println!("Checking {:?}", update);
    for to_check in 0..(update.len() - 1) {
        for elem_after in (to_check + 1)..update.len() {
            // println!(
            //     "Checking {} is not after {}",
            //     update[to_check], update[elem_after]
            // );
            if rules.contains(&Rule {
                before: update[elem_after],
                after: update[to_check],
            }) {
                println!(
                    "{} before {} is invalid!",
                    update[to_check], update[elem_after]
                );
                return false;
            }
        }
    }
    true
}

fn order_update(rules: &HashSet<Rule>, update: &[i32]) -> Vec<i32> {
    println!("Ordering {:?}", update);
    // 3 -> 5
    // 7 -> 8
    // 1 -> 3
    // 8 -> 9
    // 1,3,5,7,8,9
    let applying_rules = rules
        .iter()
        .filter(|r| update.contains(&r.before) && update.contains(&r.after))
        // update.contains(&r.after) ||
        .collect_vec();

    println!("Applying rules {:?}", applying_rules);
    // Find all starting nodes (nodes not appearing on the right side (they must come first))
    // update - rules->after
    let starting_nodes = HashSet::from_iter(update.iter().cloned())
        .difference(
            &applying_rules
                .iter()
                .map(|r| r.after)
                .collect::<HashSet<i32>>(),
        )
        .cloned()
        .collect_vec();
    println!("starting nodes: {:?}", starting_nodes);

    assert!(starting_nodes.len() == 1);
    // node -> from,weight
    let mut ans: BTreeMap<i32, Option<(i32, i32)>> = BTreeMap::new();
    let mut prio: BTreeMap<i32, i32> = BTreeMap::new();

    let following_nodes_of = |node: i32| {
        return rules
            .iter()
            .filter(|r| r.before == node && update.contains(&r.after))
            .map(|r| r.after)
            .collect_vec();
    };

    for starting_node in starting_nodes {
        ans.insert(starting_node, None);

        for next_node in following_nodes_of(starting_node) {
            ans.insert(next_node, Some((starting_node, 1)));
            prio.insert(next_node, 1);
        }

        while let Some((node, path_weight)) = prio.pop_last() {
            for next_node in following_nodes_of(node) {
                let new_weight = path_weight + 1;
                match ans.get(&next_node) {
                    // if ans[next] is a lower dist than the alternative one, we do nothing
                    Some(Some((_, dist_next))) if new_weight < *dist_next => {}
                    // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                    Some(None) => {}
                    // the new path is longer, either new was not in ans or it was farther
                    _ => {
                        ans.insert(next_node, Some((node, new_weight)));
                        prio.insert(next_node, new_weight);
                    }
                }
            }
        }
    }

    let from_to = ans
        .iter()
        .map(|(to, from)| (from.map(|f| f.0), *to))
        .collect::<HashMap<_, _>>();

    println!("result {:?} | update {:?}", ans, update);
    println!("from_to {:?}", from_to);

    let mut result = vec![];
    let mut cur = *from_to.get(&None).unwrap();
    result.push(cur);

    while let Some(n) = from_to.get(&Some(cur)) {
        cur = *n;
        result.push(*n);
    }

    println!("result {:?}", result);

    assert_eq!(result.len(), update.len());
    assert!(is_valid(rules, &result));
    result
}

fn sum_of_valid_updates(rules: &HashSet<Rule>, updates: &[Vec<i32>]) -> i32 {
    return updates
        .iter()
        .filter(|u| !u.is_empty() && !is_valid(rules, u))
        .map(|u| order_update(rules, u))
        .map(|update| {
            // valid
            println!("{:?} -> {}", update, update[update.len() / 2]);

            assert!(update.len() % 2 == 1); // not even
            update[update.len() / 2]
        })
        .reduce(|acc, n| acc + n)
        .unwrap_or(0);
}

fn main() {
    let rules_path = env::args().nth(1).expect("Usage: <file_path>");
    let updates_path = env::args().nth(2).expect("Usage: <updates_path>");

    let rules = parse_rules_from_file(&rules_path).unwrap();
    let updates = parse_updates_from_file(&updates_path).unwrap();

    println!("{}", sum_of_valid_updates(&rules, &updates));
}

// 10604 -> to high
