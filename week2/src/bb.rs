use crate::Item;

/// Uses greedy strategy to find an optimal solution to the relaxed knapsack problem.
/// The relaxation in this case is to assume that we can take fractional number of any item,
/// this allows us to find an optimal solution in O(n) time
/// (assuming input items are already sorted).
///
/// IMPORTANT: input items expected to be sorted for the result to be an optimal one.
#[inline(always)]
fn best_greedy(items: &[Item], discard: &[usize], error: f64, cap: usize) -> usize {

    let mut weight_gained = 0usize;
    let mut value_gained = 0usize;

    for (item_idx, item) in items.iter().enumerate() {

        if discard[item_idx] == 1 {
            continue
        }

        if weight_gained + item.weight > cap {

            let weight_left = cap - weight_gained;
            value_gained +=
                f64::round((weight_left * item.value) as f64 / item.weight as f64) as usize;

            break;

        } else {

            weight_gained += item.weight;
            value_gained += item.value;

            if weight_gained == cap {
                break;
            }

        }

    }

    (value_gained as f64 - value_gained as f64 * error) as usize
}

#[derive(Default, Debug)]
struct Env {
    best_val: usize,
    best_indices: Vec<usize>,

    cur_indices: Vec<usize>,
    cur_weight: usize,
    cur_val: usize,

    cap: usize,
    error: f64,
}

impl Env {

    pub(crate) fn new(items: &[Item], cap: usize, error: f64) -> Self {
        Env {
            cap,
            error,
            cur_indices: vec![0; items.len()],
            ..Default::default()
        }
    }

}

fn recur(items: &[Item], item_idx: usize, mut discard: &mut [usize], prev_est: usize, mut env: Env) -> Env {

    if item_idx == items.len() {
        return env;
    }

    let cur_item = items[item_idx];

    // discard item
    discard[item_idx] = 1;
    let est = best_greedy(items, &discard, env.error, env.cap);

    if est > env.best_val {
        // continue this branch only if there is a hope to find a better
        // solution than we've found so far
        env = recur(items, item_idx + 1, &mut discard, est, env);
    }

    // take item
    discard[item_idx] = 0;

    let est = prev_est;
    if est > env.best_val {

        // continue this branch only if there is a hope to find a better
        // solution than we've found so far

        if env.cur_weight + cur_item.weight > env.cap {
            // no enough place for the item
            return env;
        }

        env.cur_indices[cur_item.index] = 1;

        env.cur_weight += cur_item.weight;
        env.cur_val += cur_item.value;

        if env.cur_val > env.best_val {
            env.best_val = env.cur_val;
            env.best_indices = env.cur_indices.clone();
        }

        env = recur(items, item_idx + 1, &mut discard, prev_est, env);

        env.cur_indices[cur_item.index] = 0;
        env.cur_val -= cur_item.value;
        env.cur_weight -= cur_item.weight;

    }

    env

}

pub(crate) fn solve_bb(items: &mut [Item], error: f64, cap: usize) -> (usize, usize, Vec<usize>) {

    // sort items in descending order by their value per weight
    items.sort_by(|a, b|
        (b.value as f64 / b.weight as f64)
            .partial_cmp(&(a.value as f64 / a.weight as f64))
            .unwrap()
    );

    let est = best_greedy(&items, &vec![0; items.len()], error, cap);

    let res = recur(&items, 0, &mut vec![0; items.len()], est, Env::new(&items, cap, error));

    (res.best_val, if error == 0f64 { 1 } else { 0 }, res.best_indices)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_best_greedy() {

        let cap = 10;

        let items=  &vec![
            Item {
                index: 2,
                value: 35,
                weight: 3,
            },
            Item {
                index: 0,
                value: 45,
                weight: 5,
            },
            Item {
                index: 1,
                value: 48,
                weight: 8,
            },
        ];

        assert_eq!(best_greedy(items, &mut vec![0; items.len()], 0f64, cap), 92);
    }

    #[test]
    fn test_bb_simple() {

        let cap = 10;

        let items=  &vec![
            Item {
                index: 2,
                value: 35,
                weight: 3,
            },
            Item {
                index: 0,
                value: 45,
                weight: 5,
            },
            Item {
                index: 1,
                value: 48,
                weight: 8,
            },
        ];

        let est = best_greedy(items, &mut vec![0; items.len()], 0f64, cap);
        let res = recur(&items, 0, &mut vec![0; items.len()], est, Env::new(items, cap, 0f64));

        assert_eq!(res.best_val, 80);
    }

}
