use::std::collections::HashMap;

struct Solution;

impl Solution{
    pub fn frequency_sort(nums: Vec<i32>)-> Vec<i32> {
        let mut freq_map:HashMap<i32,i32>=HashMap::new();

        for &num in &nums{
            *freq_map.entry(num).or_insert(0)+=1;
        }

        let mut nums=nums;
        nums.sort_unstable_by(|a,b|{
            let freq_a=freq_map[a];
            let freq_b=freq_map[b];
            freq_a.cmp(&freq_b).then(b.cmp(a))
        });
       nums 
    }
}

//THIS IS NOT PART OF THE LEETCODE IT'S JUST TO TEST OUT MY CODE 
fn main() {
    let test_cases = vec![
        (vec![1, 1, 2, 2, 2, 3], vec![3, 1, 1, 2, 2, 2]),
        (vec![2, 3, 1, 3, 2], vec![1, 3, 3, 2, 2]),
        (vec![5, 4, 3, 2, 1], vec![5, 4, 3, 2, 1]),
        (vec![-1, -1, -2, -2, -2, -3], vec![-3, -1, -1, -2, -2, -2]),
        (vec![42], vec![42]),
        (vec![7, 7, 7, 7], vec![7, 7, 7, 7]),
        (vec![4, 4, 6, 6, 5, 5], vec![6, 6, 5, 5, 4, 4]),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::frequency_sort(input.clone());
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