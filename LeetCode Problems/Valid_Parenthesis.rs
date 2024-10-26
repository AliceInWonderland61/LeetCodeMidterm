struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack=Vec::new();

        for c in s.chars(){
            match c{
                '(' | '[' | '{' =>stack.push(c),
                ')' => if stack.pop()!=Some('('){return false},
                ']' => if stack.pop()!=Some('['){return false},
                '}' => if stack.pop()!=Some('{'){return false},
                _=>{return false},
            }
        } stack.is_empty()
    }
}
//THIS IS NOT PART OF THE LEETCODE, IT'S JUST TO TEST MY CODE OUT TO MAKE SURE IT WORKS 
fn main() {
    let test_cases = vec![
        ("()".to_string(), true),
        ("()[]{}".to_string(), true),
        ("{[()]}".to_string(), true),
        ("({[]})".to_string(), true),
        ("(]".to_string(), false),
        ("([)]".to_string(), false),
        ("{[(])}".to_string(), false),
        ("((()".to_string(), false),
        ("(()))".to_string(), false),
        ("".to_string(), true),
        ("[".to_string(), false),
        ("]".to_string(), false),
        ("{{{{[[[()]]]}}}}".to_string(), true),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::is_valid(input.clone());
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
