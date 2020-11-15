use super::sorter;

pub struct BubbleSort;

// a sorted partition will form at the end of the array
impl sorter::Sorter for BubbleSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        // this is the index that j will go up until (think of it as 'end')
        for i in 0..slice.len() {
            for j in 0..(slice.len() - i - 1) {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                }
            }
        }
    }
}

#[test]
fn even() {
    use crate::sorting_algos::sorter::Sorter;

    let mut nums = vec![4, 3, 2, 1];
    BubbleSort.sort(&mut nums);
    assert_eq!(nums, &[1, 2, 3, 4]);
}

#[test]
fn odd() {
    use crate::sorting_algos::sorter::Sorter;

    let mut nums = vec![4, 3, 5, 2, 1];
    BubbleSort.sort(&mut nums);
    assert_eq!(nums, &[1, 2, 3, 4, 5]);
}
