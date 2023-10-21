pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries
        .iter()
        .map(|query| {
            let r = query[2] * query[2];
            points
                .iter()
                .filter(|point| {
                    let x = point[0] - query[0];
                    let y = point[1] - query[1];
                    x * x + y * y <= r
                })
                .count() as i32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_points() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let queries = vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]];
        assert_eq!(count_points(points, queries), vec![2, 3, 2, 4])
    }
}
