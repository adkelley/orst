use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice [..=unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 | ]
            // [ 1 3 2 4 | ]
            // [ 1 2 3 4 | ]
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i, i - 1);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                // then use .rotate_right to splice in i
                // slice[..unsorted] is sorted
                // let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                //     // [ a, c, d].binary_search(c) => Ok(1)
                //     // [ a, c, e].binary_search(b) => Err(1)
                //     Ok(i) => i,
                //     Err(i) => i,
                // };
                let (Ok(i) | Err(i)) = slice[..unsorted].binary_search(&slice[unsorted]);
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_dumb() {
    let mut things = vec![4, 3, 5, 2, 1];
    // super::sort::<_, Insertionsort>(&mut things);
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 3, 5, 2, 1];
    // super::sort::<_, Insertionsort>(&mut things);
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
