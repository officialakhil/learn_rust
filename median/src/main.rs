fn merge_and_find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() && nums2.is_empty() {
        return 0.0;
    }

    let mut nums: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();

    nums.sort_unstable();

    let len = nums.len();

    if len % 2 == 0 {
        let mid = len / 2;
        (nums[mid] + nums[mid - 1]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn main() {
    let nums1 = vec![1, 3, 2, 6, -4];
    let nums2 = vec![2, 9, 1, 4, 5];
    println!("{}", merge_and_find_median(nums1, nums2));
}

#[cfg(test)]
mod tests {
    use super::merge_and_find_median;

    #[test]
    fn test_odd() {
        let nums1 = vec![1, 3, 2, 6, -4];
        let nums2 = vec![2, 9, 1, 4];
        assert_eq!(merge_and_find_median(nums1, nums2), 2.0);
    }

    #[test]
    fn test_even() {
        let nums1 = vec![1, 3, 2, 6, -4];
        let nums2 = vec![2, 9, 1, 4, 5];
        assert_eq!(merge_and_find_median(nums1, nums2), 2.5);
    }

    #[test]
    fn test_empty() {
        let nums1 = vec![];
        let nums2 = vec![];
        assert_eq!(merge_and_find_median(nums1, nums2), 0.0);
    }
}
