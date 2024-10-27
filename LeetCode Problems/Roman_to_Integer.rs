use std::collections::HashMap;
struct Solution;
impl Solution{
pub fn roman_to_int(s:String)->i32{
    let mut roman_values=HashMap::new();
    roman_values.insert('I',1);
    roman_values.insert('V',5);
    roman_values.insert('X',10);
    roman_values.insert('L',50);
    roman_values.insert('C', 100);
    roman_values.insert('D',500);
    roman_values.insert('M', 1000);

    let mut prev_value=0;
    let mut result=0;

    for ch in s.chars().rev(){
        let current_value=roman_values[&ch];

        if current_value>=prev_value{
            result+=current_value;
        }
        else
        {
            result-=current_value;
        }
        prev_value=current_value;
    }
result
    
}
}

fn main() {
    let test_cases = vec![
        ("III".to_string(), 3),
        ("LVIII".to_string(), 58),
        ("IX".to_string(), 9),
        ("MCMXCIV".to_string(), 1994),
        ("CDXLIV".to_string(), 444),
        ("I".to_string(), 1),
        ("X".to_string(), 10),
        ("M".to_string(), 1000),
        ("MMM".to_string(), 3000),
        ("CCCLXXXVIII".to_string(), 388),
        ("".to_string(), 0),
        ("IV".to_string(), 4),
        ("CM".to_string(), 900),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::roman_to_int(input.clone());
        println!(
            "Test case {}: input = {:?} | Expected = {}, Got = {}",
            i + 1,
            input,
            expected,
            result
        );
        assert_eq!(result, *expected);
    }

    println!("All test cases passed!");
}