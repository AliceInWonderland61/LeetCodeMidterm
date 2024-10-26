struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s=x.to_string();
        let bytes=s.as_bytes();

        let mut left=0;
        let mut right=bytes.len()-1;

        while left<right{
            if bytes[left]!=bytes[right]
            {
                return false;
            }
            else
            {
                left=left+1;
                right=right-1;
            }
        }
        return true
    }
}
//NOT PART OF LEETCODE, IT'S JUST TO TEST MY SOLUTION 
fn main() {
    let test_cases = vec![
        (121, true),
        (12321, true),
        (0, true),
        (10, false),
        (123, false),
        (12310, false),
        (-121, false),
        (-1, false),
        (123454321, true),
        (1000000001, true),
        (1234567899, false),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::is_palindrome(*input);
        println!("Test case {}: input = {}, Expected = {}, Got = {}", 
                 i + 1, input, expected, result);
        assert_eq!(result, *expected);
    }

    println!("All test cases passed!");
}
