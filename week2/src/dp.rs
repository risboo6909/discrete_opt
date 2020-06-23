use std::cmp::max;
use fxhash::FxHashMap;

use crate::Item;


fn reverse(dp: FxHashMap<(usize, usize), usize>, items: &[Item], mut last_cell: (usize, usize)) -> (usize, Vec<usize>) {

    let mut res = vec![0; items.len()];
    let mut value = 0;

    loop {

        let (cur_idx, cur_cap) = last_cell;

        if cur_idx == 0 {
            break
        }

        let cur_v = dp.get(
            &last_cell
        ).unwrap_or(&0usize);

        let prev_v = dp.get(
            &(cur_idx - 1, cur_cap)
        ).unwrap_or(&0usize);

        if cur_v != prev_v {
            let tmp = &items[cur_idx - 1];
            last_cell = (cur_idx - 1, cur_cap - tmp.weight);
            // items were sorted before therefor we have to restore their original positions
            res[tmp.index] = 1;
            value += tmp.value;
        } else {
            last_cell = (cur_idx - 1, cur_cap);
        }

    }

    (value, res)

}

/// Dynamic programming approach, basically for each item we have 2 possibilities:
///
/// Vj + O(j-1, cap-Wj) if we put item j into knapsack
/// O(j-1, Wj) if we skip item j
///
/// I will use hashmap for saving values, it is slower but requires less memory
pub(crate) fn solve(items: &mut [Item], cap: usize) -> (usize, usize, Vec<usize>) {

    // use hashmap because we don't need to save 0 cells
    let mut dp = FxHashMap::with_capacity_and_hasher(items.len() * cap / 2, Default::default());

    let mut last_cell = (0, 0);

    // sort such that heaviest items go first, to save memory (we don't save 0 cells, i.e.
    // cells which are heavier than current capacity)
    items.sort_by_key(|item| -(item.weight as isize));

    for (item_idx, item) in items.iter().enumerate() {

        if item.weight > cap {
            continue
        }

        // we can start from item.weight, because all items are sorted earlier by their weights
        // and absence of a key in a map assumes 0 value
        for cur_cap  in item.weight..=cap {

            // consider two cases, whether we take an item or not and choose one
            // with the maximal final weight
            let ignore = *dp.get(
                &(item_idx, cur_cap)
            ).unwrap_or(&0usize);

            let take = if cur_cap >= item.weight {
                dp.get(
                    &(item_idx, cur_cap - item.weight)
                ).unwrap_or(&0usize) + item.value
            } else {
                0
            };

            last_cell = (item_idx + 1, cur_cap);

            let w = max(ignore, take);
            if w > 0 {
                // don't waste memory saving zeroes
                dp.insert(last_cell, w);
            }

        }

    }

    // return result value, optimality (1 - optimal, 0 - not opt) and taken items array
    let (res_val, items_taken) = reverse(dp, items, last_cell);

    (res_val, 1, items_taken)
}
