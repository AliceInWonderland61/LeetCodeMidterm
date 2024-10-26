struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut iter1=word1.chars();
        let mut iter2=word2.chars();
        let mut merged=String::new();

        loop
        {
            if let Some(c1)=iter1.next(){
                merged.push(c1);
            }
            if let Some(c2)= iter2.next(){
                merged.push(c2)
            }

            if iter1.as_str().is_empty() && iter2.as_str().is_empty()
            {
                break;
            }
        }
        merged
    }
} 

//NOT PART OF LEETCODE, JUST SOME TESTS TO MAKE SURE MY CODE WORKS 
fn main() {
    let test_cases = vec![
        (("abc".to_string(), "pqr".to_string()), "apbqcr".to_string()),
        (("ab".to_string(), "pqrs".to_string()), "apbqrs".to_string()),
        (("abcd".to_string(), "pq".to_string()), "apbqcd".to_string()),
        (("".to_string(), "".to_string()), "".to_string()),
        (("".to_string(), "xyz".to_string()), "xyz".to_string()),
        (("hello".to_string(), "".to_string()), "hello".to_string()),
        (("a".to_string(), "z".to_string()), "az".to_string()),
        (("abcdef".to_string(), "123".to_string()), "a1b2c3def".to_string()),
    ];

    for (i, ((word1, word2), expected)) in test_cases.iter().enumerate() {
        let result = Solution::merge_alternately(word1.clone(), word2.clone());
        println!("Test case {}: word1 = {:?}, word2 = {:?} | Expected = {:?}, Got = {:?}", 
                 i + 1, word1, word2, expected, result);
        assert_eq!(&result, expected);
    }

    println!("All test cases passed!");
}


