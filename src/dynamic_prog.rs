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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution1() {
        let trade = vec![4, 11, 2, 1, 7];
        let res = max_profit(trade);
        println!("res: {}", res);
        assert_eq!(7, res);
    }

    #[test]
    fn solution2() {
        let trade = vec![4, 11, 2, 1, 7];
        let res = max_profit2(trade);
        println!("res: {}", res);
        assert_eq!(7, res);
    }
}
