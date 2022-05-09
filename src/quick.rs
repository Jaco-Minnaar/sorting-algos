pub fn quick_sort<T>(nums: &mut [T])
where
    T: PartialOrd + Copy,
{
    sort(nums, 0, nums.len() - 1);
}

fn sort<T>(nums: &mut [T], low: usize, high: usize)
where
    T: PartialOrd + Copy,
{
    if low >= high {
        return;
    }

    let pi = partition(nums, low, high);

    sort(nums, low, pi.saturating_sub(1));
    sort(nums, pi + 1, high);
}

fn partition<T>(nums: &mut [T], low: usize, high: usize) -> usize
where
    T: PartialOrd + Copy,
{
    let pivot = nums[high];
    let mut i = low - 1;

    for j in low..high - 1 {
        if nums[j] < pivot {
            i += 1;

            nums.swap(i, j);
        }
    }

    nums.swap(i + 1, high);

    i + 1
}
