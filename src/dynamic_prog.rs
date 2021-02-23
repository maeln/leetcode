use std::vec;

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// The actual good version where we iterate over the prices only once.
pub fn max_profit2(prices: Vec<i32>) -> i32 {
    let mut min_buy_val = i32::max_value();
    let mut profit = 0;
    for i in 0..prices.len() {
        if min_buy_val > prices[i] {
            min_buy_val = prices[i];
        } else if prices[i] - min_buy_val > profit {
            profit = prices[i] - min_buy_val;
        }
    }
    return profit;
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// this is a very bad and overly complicated answer
// it's a very optimized version of a simple bruteforce that try to pass all the value that it knows cannot be used.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut profit = 0;
    let mut day_buy_val = i32::max_value();
    let mut max_sell_idx = 0;
    let mut max_sell_value = 0;

    for buy in 0..(prices.len() - 1) {
        let buy_val = prices[buy];
        // no need to continue this iteration if we cannot buy lower.
        if day_buy_val <= buy_val {
            continue;
        }
        day_buy_val = buy_val;
        println!("New min buy val: {}, idx {}", day_buy_val, buy);

        // if the hightest sell day is within range, we use it
        if max_sell_idx > buy {
            let new_profit = max_sell_value - buy_val;
            if new_profit > profit {
                profit = new_profit;
                println!("sell idx {} valid, profit {}", max_sell_idx, profit);
            }
        } else {
            // otherwise we have to find the new max in the range.
            println!("sell idx {} invalid, looking for new max.", max_sell_idx);
            let mut msi: i32 = -1;
            let mut msv = buy_val;
            for sell in (buy + 1)..prices.len() {
                if prices[sell] > msv {
                    msv = prices[sell];
                    msi = sell as i32;
                    println!("found new max {} at idx {}", msv, msi);
                }
            }
            if msi != -1 {
                max_sell_idx = msi as usize;
                max_sell_value = msv;
                let new_profit = max_sell_value - buy_val;
                if new_profit > profit {
                    profit = new_profit;
                    println!("new profit {}", profit);
                }
            } else {
                println!("Could not find new max.");
            }
        }
    }

    return profit;
}

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/97/dynamic-programming/569/
// recursive solution. Works but really slow and expensive.
pub fn climb_stairs_rec(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => climb_stairs_rec(n - 1) + climb_stairs_rec(n - 2),
    }
}

// stack based, still too slow
pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut count = 0;
    let mut stack: Vec<i32> = vec![n];
    while !stack.is_empty() {
        let val = stack.pop().unwrap();
        if val == 1 {
            count += 1;
        } else if val == 2 {
            count += 2;
        } else {
            stack.push(val - 1);
            stack.push(val - 2);
        }
    }
    return count;
}

// Dynamic prog version
pub fn climb_stairs_dyn(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut steps: Vec<i32> = vec![0; n as usize + 1];
    steps.insert(1, 1);
    steps.insert(2, 2);
    for i in 3..(n as usize + 1) {
        steps.insert(i, steps[i - 1] + steps[i - 2]);
    }
    return steps[n as usize];
}

// LUT based one, just for fun.
const LUT: &[i32] = &[
    0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170, 1836311903,
];
pub fn climb_stairs_lut(n: i32) -> i32 {
    return LUT[n as usize];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stocks_solution1() {
        let trade = vec![4, 11, 2, 1, 7];
        let res = max_profit(trade);
        println!("res: {}", res);
        assert_eq!(7, res);
    }

    #[test]
    fn stocks_solution2() {
        let trade = vec![4, 11, 2, 1, 7];
        let res = max_profit2(trade);
        println!("res: {}", res);
        assert_eq!(7, res);
    }

    #[test]
    fn climbing_stairs_rec() {
        assert_eq!(2, climb_stairs_rec(2));
        assert_eq!(3, climb_stairs_rec(3));
    }

    #[test]
    fn climbing_stairs() {
        assert_eq!(0, climb_stairs(0));
        assert_eq!(1, climb_stairs(1));
        assert_eq!(2, climb_stairs(2));
        assert_eq!(3, climb_stairs(3));
    }

    #[test]
    fn climbing_stairs_dyn() {
        assert_eq!(0, climb_stairs_dyn(0));
        assert_eq!(1, climb_stairs_dyn(1));
        assert_eq!(2, climb_stairs_dyn(2));
        assert_eq!(3, climb_stairs_dyn(3));
        assert_eq!(1836311903, climb_stairs_dyn(45));
    }

    #[test]
    fn climbing_stairs_lut() {
        assert_eq!(0, climb_stairs_lut(0));
        assert_eq!(1, climb_stairs_lut(1));
        assert_eq!(2, climb_stairs_lut(2));
        assert_eq!(3, climb_stairs_lut(3));
        assert_eq!(1836311903, climb_stairs_lut(45));
    }
}
