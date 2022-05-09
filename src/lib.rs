use rand::{distributions::Standard, prelude::Distribution};

pub mod bubble;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;

pub fn generate_nums<T>(length: usize) -> Vec<T>
where
    Standard: Distribution<T>,
    T: Copy + Clone + Default,
{
    let mut vec: Vec<T> = vec![T::default(); length];

    for i in 0..length {
        let rand_num: T = rand::random();

        vec[i] = rand_num;
    }

    vec
}

pub fn check_if_sorted<T>(nums: &[T]) -> bool
where
    T: PartialOrd,
{
    let mut sorted = true;
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            sorted = false;
            break;
        }
    }

    sorted
}
