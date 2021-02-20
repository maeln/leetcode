// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// remove duplicate  from sorted array
// this approach is working but use slightly more memory.
// we can avoid the unique indice arrayy by iterating through or vector in reverse
// and looking a the value a i and i-1 and shifting the values.
pub fn remove_duplicates_sorted_array(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut last = nums[0];
    let mut unique_idx: Vec<usize> = vec![0];
    for i in 1..nums.len() {
        if last != nums[i] {
            unique_idx.push(i)
        }
        last = nums[i];
    }
    for i in 0..unique_idx.len() {
        nums[i] = nums[unique_idx[i]];
    }
    nums.resize(unique_idx.len(), 0);
    return unique_idx.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution1() {
        let mut sorted = vec![1, 1, 2, 2, 2, 3, 4, 5, 5, 6];
        let res = remove_duplicates_sorted_array(&mut sorted);
        assert_eq!(6, res);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6])
    }
}
