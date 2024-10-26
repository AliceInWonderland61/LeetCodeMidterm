struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty()
        {
            return 0;
        }
       
        let mut min_price=i32::MAX;
        let mut max_profit=0;

        for price in prices{
            if price< min_price{
                min_price=price;
            }
            let mut profit=price-min_price;

            if profit>max_profit{
                max_profit=profit;
            }
        }
        max_profit
    }
}
// THIS ISN'T PART OF THE LEETCODE I JUST WANTED TO MAKE SURE THE CODE WORKED WITH SOME RANDOM TEST CASES 
fn main() {
    let test_cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),    // Buy at 1, sell at 6
        (vec![7, 6, 4, 3, 1], 0),       // Prices decrease, no profit
        (vec![2, 4, 1], 2),             // Buy at 2, sell at 4
        (vec![3, 3, 5, 0, 0, 3, 1, 4], 4),  // Buy at 0, sell at 4
        (vec![1, 2], 1),                // Buy at 1, sell at 2
        (vec![2, 1], 0),                // Prices decrease, no profit
        (vec![], 0),                    // Empty prices, no profit
        (vec![1], 0),                   // Single price, no transactions possible
        (vec![1, 2, 3, 4, 5, 6, 7], 6), // Increasing prices, buy at 1, sell at 7
    ];

    for (i, (prices, expected)) in test_cases.iter().enumerate() {
        let result = Solution::max_profit(prices.clone());
        println!("Test case {}: prices = {:?} | Expected = {}, Got = {}", 
                 i + 1, prices, expected, result);
        assert_eq!(result, *expected);
    }

    println!("All test cases passed!");
}