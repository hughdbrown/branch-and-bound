use std::cmp::max;

use knapsack_utils::{
    Item,
    SearchResult,
    // sum_weights,
    sum_values,
};

fn branch_and_bound_helper(
    items: &[Item],
    i: usize,
    limit_weight: usize,
    weight: usize,
    value: u64,
    //mut max_weight: Box<Result<usize, ()>>,
    mut max_value: u64,
    path: &[usize],
) -> Result<SearchResult, ()> {
    if weight > limit_weight {
        //println!("{weight} > {limit_weight}");
        return Err(());
    }
    if i >= items.len() {
        let r: SearchResult = (path.to_vec(), weight, value);
        //println!("Success: {weight}/{limit_weight} {value}");
        return Ok(r);
    }

    let remaining_items = &items[i..];
    let remaining_value = sum_values(remaining_items);
    let left: Result<SearchResult, ()> = if value + remaining_value > max_value {
        // Try with item[i]
        // Make a new copy of the immutable path argument that has `i` appended.
        let mut lpath: Vec<usize> = path.to_vec();
        lpath.push(i);
        let item = &items[i];
        branch_and_bound_helper(
            items,
            i + 1,
            limit_weight,
            weight + item.weight,
            value + item.value,
            //max_weight.clone(),
            max_value,
            &lpath)

    }
    else {
        Err(())
    };

    match left {
        Ok(ref t) => { max_value = max(max_value, t.2); }
        Err(_) => {},
    }

    let remaining_items = &items[i+1..];
    let remaining_value = sum_values(remaining_items);
    let right: Result<SearchResult, ()> = if value + remaining_value > max_value {
        // Try without item[i]
        branch_and_bound_helper(
            items,
            i + 1,
            limit_weight,
            weight,
            value,
            //max_weight,
            max_value,
            path)
    }
    else {
        Err(())
    };

    // Which is better?
    match (left, right) {
        (Ok(lvalue), Ok(rvalue)) => {
            let best_value = if lvalue.2 > rvalue.2 { lvalue } else { rvalue };
            return Ok(best_value);
        },
        (Ok(lvalue), Err(())) => return Ok(lvalue),
        (Err(()), Ok(rvalue)) => return Ok(rvalue),
        (Err(()), Err(())) => return Err(()),
    }
}

pub fn branch_and_bound(items: &[Item], limit_weight: usize) -> Result<SearchResult, ()> {
    let path = vec![];
    return branch_and_bound_helper(
        items,
        0, // i = 0 (start at the first item)
        limit_weight,
        0, // weight = 0 (Weight of current path)
        0, // value = 0 (Value of current path)
        //Box::new(max_weight), // max_weight seen on completed branch
        0, // max_value seen on completed branch
        &path); // Current path (starts empty)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_and_bound_1() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        match branch_and_bound(&items, 4) {
            Ok(value) => assert_eq!(value.2, 3500),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn branch_and_bound_2() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
            Item{value:3300, weight:5},
            Item{value:4000, weight:6},
            Item{value:4200, weight:7},
            Item{value:4400, weight:8},
        ];
        let expected_vals: Vec<u64> = vec![22400, 20900, 20400, 20400, 19400, 19100, 18400, 18200];
        for (i, expected_val) in expected_vals.iter().map(|x: &u64| *x).enumerate() {
            println!("----- Testing {i} for value {expected_val}");
            match branch_and_bound(&items, 34 - i) {
                Ok(value) => assert_eq!(value.2, expected_val),
                Err(_) => { },
            }
        }
    }
}
