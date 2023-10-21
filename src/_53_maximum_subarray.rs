
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0, 0), |(max, sum), &x| {
        // 計算以當前項為結尾的最大子陣列和
        // 如果當前項比以前一項結束子序列和大，則以當前項取代計算結果
        let sum = if sum + x > x { sum + x } else { x };
        // 暫存最大子陣列和
        let max = if max > sum { max } else { sum };
        (max, sum)
    }).0
}

#[cfg(test)]
mod test
{
    use crate::_53_maximum_subarray::max_sub_array;

    #[test]
    fn test_1() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let expect = 6;

        assert_eq!(max_sub_array(input), expect);
    }

}
