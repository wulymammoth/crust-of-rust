pub trait Sorter {
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 3, 2, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
