struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        if words.is_empty(){
            return Vec::new();
        }
        let first_word=&words[0];
        let mut result=Vec::new();

        for ch in first_word.chars(){
            if words.iter().skip(1).all(|word|{
                let remaining_occurances=word.chars().filter(|&c|c==ch).count();
                let result_occurrences=result.iter().filter(|&s|s==&ch.to_string()).count();
                remaining_occurances>result_occurrences

            })
            {
                result.push(ch.to_string());
            }

        }
        result
    }
}

//NOT PART OF LEETCODE, JUST TESTING OUT MY CODE TO MAKE SURE IT WORKS
fn main() {
    let test_cases = vec![
        (vec!["bella".to_string(), "label".to_string(), "roller".to_string()], vec!["e".to_string(), "l".to_string(), "l".to_string()]),
        (vec!["cool".to_string(), "lock".to_string(), "cook".to_string()], vec!["c".to_string(), "o".to_string()]),
        (vec!["dog".to_string(), "racecar".to_string(), "car".to_string()], vec![]),
        (vec!["apple".to_string(), "apple".to_string(), "apple".to_string()], vec!["a".to_string(), "p".to_string(), "p".to_string(), "l".to_string(), "e".to_string()]),
        (vec!["aaa".to_string(), "aa".to_string(), "aaaa".to_string()], vec!["a".to_string(), "a".to_string()]),
        (vec!["hello".to_string()], vec!["h".to_string(), "e".to_string(), "l".to_string(), "l".to_string(), "o".to_string()]),
        (vec![], vec![]),
        (vec!["abc".to_string(), "".to_string(), "bca".to_string()], vec![]),
        (vec!["Abc".to_string(), "abc".to_string(), "ABC".to_string()], vec![]),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::common_chars(input.clone());
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