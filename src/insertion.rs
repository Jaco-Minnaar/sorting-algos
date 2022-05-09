use std::fmt::Debug;

pub fn insertion_sort<T>(nums: &mut [T])
where
    T: PartialOrd + Copy + Debug,
{
    for i in 1..nums.len() {
        let current = nums[i];
        for j in (0..i).rev() {
            if current > nums[j] {
                move_value(nums, i, j + 1);
                break;
            } else if j == 0 {
                move_value(nums, i, 0);
            }
        }
    }
}

fn move_value<T>(nums: &mut [T], from: usize, to: usize)
where
    T: Copy,
{
    if to > from {
        unimplemented!();
    }

    let moving = nums[from];

    for i in (to + 1..=from).rev() {
        nums[i] = nums[i - 1];
    }

    nums[to] = moving;
}
