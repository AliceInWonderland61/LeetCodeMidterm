use std::collections::HashMap;


    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, usize>=HashMap::new();
        for (i, &num) in nums.iter().enumerate(){
            let  complement=target-num;
            if let Some(&complement_index)=hash.get(&complement){ 
            return vec![complement_index as i32, i as i32];
        }
        
        hash.insert(num,i);

    }
    vec![]
}

/// this isn't part of the leetcode submission it's just testing it out so that I can run it here in codespace 
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}