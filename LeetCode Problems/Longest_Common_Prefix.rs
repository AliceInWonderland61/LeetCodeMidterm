struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty(){
            return "".to_string();
        }
        let mut prefix=strs[0].clone();
        
        for s in &strs[1..]{
            while !s.starts_with(&prefix)
            {
                prefix.pop();
                
                if prefix.is_empty()
                {
                    return "".to_string();
                }
            }
        }
        prefix
    }
}
//THIS IS NOT PART OF THE LEETCODE, IT'S JUST TO TEST MY CODE OUT AND MAKE SURE IT WORKS
fn main() {
    let test_cases = vec![
        (vec!["flower".to_string(), "flow".to_string(), "flight".to_string()], "fl".to_string()),
        (vec!["dog".to_string(), "racecar".to_string(), "car".to_string()], "".to_string()),
        (vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()], "".to_string()),
        (vec!["same".to_string(), "same".to_string(), "same".to_string()], "same".to_string()),
        (vec!["test".to_string()], "test".to_string()), // single string case
        (vec!["".to_string(), "prefix".to_string(), "pre".to_string()], "".to_string()), // empty string in list
        (vec![], "".to_string()), // empty list case
        (vec!["".to_string()], "".to_string()), // list with single empty string
        (vec!["prefix".to_string(), "preach".to_string(), "present".to_string()], "pre".to_string()),
        (vec!["abcde".to_string(), "abcd".to_string(), "abc".to_string(), "ab".to_string()], "ab".to_string()), // progressively shorter common prefix
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::longest_common_prefix(input.clone());
        println!(
            "Test case {}: input = {:?} | Expected = {:?}, Got = {:?}",
            i + 1,
            input,
            expected,
            result
        );
        assert_eq!(result, *expected);
    }

    println!("All test cases passed!");
}