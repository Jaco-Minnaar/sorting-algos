use sorting::{bubble, check_if_sorted, generate_nums, insertion, merge, quick, selection};
use std::time::Instant;

fn main() {
    let nums = generate_nums::<u8>(1_000_000);

    // =================================================
    // Std Sort
    // =================================================

    let mut std_vec = nums.clone();
    let now = Instant::now();
    std_vec.sort();

    let duration = now.elapsed();

    if check_if_sorted(&std_vec) {
        println!("Vec::sort sort: {} us", duration.as_micros());
    } else {
        println!("Vec::sort sort: Not sorted");
    }

    // =================================================
    // Merge Sort
    // =================================================

    let mut merge_vec = nums.clone();
    let now = Instant::now();
    merge::merge_sort(merge_vec.as_mut_slice());

    let duration = now.elapsed();

    if check_if_sorted(&merge_vec) {
        println!("Merge sort: {} us", duration.as_micros());
    } else {
        println!("Merge sort: Not sorted");
    }

    // =================================================
    // Quick Sort
    // =================================================

    let mut quick_vec = nums.clone();
    let now = Instant::now();
    quick::quick_sort(quick_vec.as_mut_slice());

    let duration = now.elapsed();

    if check_if_sorted(&merge_vec) {
        println!("Quick sort: {} us", duration.as_micros());
    } else {
        println!("Quick sort: Not sorted");
    }

    // =================================================
    // Insertion Sort
    // =================================================

    let mut insertion_vec = nums.clone();

    let now = Instant::now();
    insertion::insertion_sort(insertion_vec.as_mut_slice());

    let duration = now.elapsed();

    if check_if_sorted(&insertion_vec) {
        println!("Insertion sort: {} us", duration.as_micros());
    } else {
        println!("Insertion sort: Not sorted");
    }

    // =================================================
    // Selection Sort
    // =================================================

    let mut selection_vec = nums.clone();
    let now = Instant::now();
    selection::selection_sort(selection_vec.as_mut_slice());

    let duration = now.elapsed();

    if check_if_sorted(&selection_vec) {
        println!("Selection sort: {} us", duration.as_micros());
    } else {
        println!("Selection sort: Not sorted");
    }

    // =================================================
    // Bubble Sort
    // =================================================

    let mut bubble_vec = nums.clone();
    let now = Instant::now();
    bubble::bubble_sort(bubble_vec.as_mut_slice());

    let duration = now.elapsed();

    if check_if_sorted(&bubble_vec) {
        println!("Bubble sort: {} us", duration.as_micros());
    } else {
        println!("Bubble sort: Not sorted");
    }
}
