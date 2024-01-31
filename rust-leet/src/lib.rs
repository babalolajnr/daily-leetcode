pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize; // Convert m to usize

    let first_m_elements: Vec<i32> = nums1.iter().take(m).cloned().collect();

    let mut my_arr: Vec<i32> = Vec::new();
    my_arr.extend(first_m_elements);
    my_arr.extend(nums2.iter().cloned());

    my_arr.sort();

    *nums1 = my_arr;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn merge_test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn merge_test_case_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }
}
