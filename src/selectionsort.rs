use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            // find the min element in the unsorted a[i..a.len()-1]
            let (smallest_in_rest, _) = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _v)| (unsorted + i, _v))
                .expect("slice is non-empty");

            // or

            // let mut smallest_in_rest = unsorted;
            // for j in (unsorted + 1)..slice.len() {
            //     if slice[j] < slice[smallest_in_rest] {
            //         smallest_in_rest = j;
            //     }
            // }

            if smallest_in_rest != unsorted {
                slice.swap(unsorted, smallest_in_rest)
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    // super::sort::<_, SelectionSort>(&mut things);
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
