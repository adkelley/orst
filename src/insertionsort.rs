use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut i = 1;
        while i < slice.len() {
            let mut j = i;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j, j - 1);
                j -= 1;
            }
            i += 1;
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    // super::sort::<_, Insertionsort>(&mut things);
    InsertionSort::sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
