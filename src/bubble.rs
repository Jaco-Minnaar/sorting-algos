pub fn bubble_sort<T>(nums: &mut [T])
where
    T: PartialOrd + Copy,
{
    let length = nums.len();

    for i in 0..length {
        for j in 0..(length - i - 1) {
            if nums[j] > nums[j + 1] {
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
            }
        }
    }
}
