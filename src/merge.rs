pub fn merge_sort<T>(nums: &mut [T])
where
    T: PartialOrd + Copy,
{
    sort(nums, 0, nums.len() - 1);
}

fn sort<T>(nums: &mut [T], start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    if start >= end {
        return;
    }

    let mid = start + (end - start) / 2;

    // println!("start: {}\nmid: {}\nend: {}\n", start, mid, end);

    sort(nums, start, mid);
    sort(nums, mid + 1, end);
    merge(nums, start, mid, end)
}

fn merge<T>(nums: &mut [T], left: usize, mid: usize, right: usize)
where
    T: PartialOrd + Copy,
{
    let left_sl = nums[left..=mid].to_vec();
    let right_sl = nums[mid + 1..=right].to_vec();

    let mut left_i = 0;
    let mut right_i = 0;
    let mut merged_i = left;

    while left_i < left_sl.len() && right_i < right_sl.len() {
        nums[merged_i] = if left_sl[left_i] <= right_sl[right_i] {
            left_i += 1;
            left_sl[left_i - 1]
        } else {
            right_i += 1;
            right_sl[right_i - 1]
        };

        merged_i += 1;
    }

    while left_i < left_sl.len() {
        nums[merged_i] = left_sl[left_i];
        left_i += 1;
        merged_i += 1;
    }

    while right_i < right_sl.len() {
        nums[merged_i] = right_sl[right_i];
        right_i += 1;
        merged_i += 1;
    }
}
