pub fn selection_sort<T>(nums: &mut [T])
where
    T: PartialOrd + Copy,
{
    let length = nums.len();

    for i in 0..length {
        let mut highest: Option<T> = None;
        let mut highest_index = 0;
        for j in 0..(length - i) {
            if let Some(highest_num) = highest {
                if nums[j] > highest_num {
                    highest.replace(nums[j]);
                    highest_index = j;
                }
            } else {
                highest.replace(nums[j]);
            }
        }

        nums[highest_index] = nums[length - i - 1];
        nums[length - i - 1] = highest.unwrap();
    }
}
