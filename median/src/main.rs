use std::io::ErrorKind;

fn merge_and_find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> Result<f64, ErrorKind> {
    if nums1.is_empty() && nums2.is_empty() {
        return Err(ErrorKind::InvalidInput);
    }

    let mut nums: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();

    nums.sort_unstable();

    let len = nums.len();

    if len % 2 == 0 {
        let mid = len / 2;
        Ok((nums[mid] + nums[mid - 1]) as f64 / 2.0)
    } else {
        Ok(nums[len / 2] as f64)
    }
}

fn main() {
    let nums1 = vec![1, 3, 2, 6, -4];
    let nums2 = vec![2, 9, 1, 4, 5];
    println!(
        "Median of two arrays is {}",
        merge_and_find_median(nums1, nums2).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() -> Result<(), ErrorKind> {
        let nums1 = vec![1, 3, 2, 6, -4];
        let nums2 = vec![2, 9, 1, 4];
        assert_eq!(merge_and_find_median(nums1, nums2)?, 2.0);

        Ok(())
    }

    #[test]
    fn test_even() -> Result<(), ErrorKind> {
        let nums1 = vec![1, 3, 2, 6, -4];
        let nums2 = vec![2, 9, 1, 4, 5];
        assert_eq!(merge_and_find_median(nums1, nums2)?, 2.5);

        Ok(())
    }

    #[test]
    fn test_empty() {
        assert!(
            merge_and_find_median(vec![], vec![]).is_err_and(|err| err == ErrorKind::InvalidInput)
        );
    }
}
