struct Solution;

impl Solution{
    pub fn climb_stairs(n: i32)->i32{
        if n<=2{
            return n;
        }

        let mut prev=1;
        let mut curr=2;
        for _ in 3..=n{
            let next=prev+curr;
            prev=curr;
            curr=next;
        }
        curr
    }
}

//NOT PART OF THE LEETCODE, THIS IS JUST TO MAKE SURE MY CODE WORKS
fn main() {
    let test_cases = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 5),
        (5, 8),
        (10, 89),
        (15, 987),
        (30, 1346269),
        (45, 1836311903),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result = Solution::climb_stairs(*input);
        println!(
            "Test case {}: input = {} | Expected = {}, Got = {}",
            i + 1,
            input,
            expected,
            result
        );
        assert_eq!(result, *expected);
    }

    println!("All test cases passed!");
}