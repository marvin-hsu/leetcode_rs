pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    // let mut dp = vec![1; nums.len()];
    //
    // for i in 0..nums.len(){
    //     for j in 0..i {
    //         if nums[j] < nums[i] {
    //             dp[i] = dp[i].max(dp[j]+1);
    //         }
    //     }
    // }

    // dp.iter().max().unwrap_or(&0).clone()

    nums.iter()
        .fold((Vec::new(), 0), |(mut ddp, max), item| {
            let i = ddp
                .iter()
                .zip(nums.iter())
                .filter(|(_, y)| y < &item)
                .map(|x| x.0)
                .max()
                .unwrap_or(&0)
                + 1;
            ddp.push(i);
            (ddp, i.max(max))
        })
        .1
}

#[cfg(test)]
mod test {
    use crate::_300_longest_increasing_subsequence::length_of_lis;

    #[test]
    fn test_1() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let expect = 4;

        assert_eq!(length_of_lis(input), expect);
    }
}
