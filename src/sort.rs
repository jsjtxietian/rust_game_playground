use std::f32::consts::E;

use bevy::utils::tracing::Instrument;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

pub struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;

            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

pub struct SelectionSort;
impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("Slice is non-empty!");

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

fn quicksort<T>(slice: &mut [T])
where
    T: Ord,
{
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        } else {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        }
    }
    // re-align left to account for the pivot at 0
    let left = left + 1;
    // place the pivot at its final location
    slice.swap(0, left - 1);

    // split_at_mut(mid: usize) -> (&mut [..mid), &mut [mid..])
    let (left, right) = slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}

pub struct QuickSort;
impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    #[test]
    fn bubble() {
        let mut things = vec![4, 2, 5, 3, 1];
        BubbleSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_works_dumb() {
        let mut things = vec![4, 2, 5, 3, 1];
        InsertionSort { smart: false }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_works_smart() {
        let mut things = vec![4, 2, 5, 3, 1];
        InsertionSort { smart: true }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection() {
        let mut things = vec![4, 2, 5, 3, 1];
        SelectionSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn quick() {
        let mut things = vec![4, 2, 5, 3, 1];
        QuickSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}

// use orst::*;

// use rand::prelude::*;
// use std::cell::Cell;
// use std::cmp::Ordering;
// use std::rc::Rc;

// #[derive(Clone)]
// struct SortEvaluator<T> {
//     t: T,
//     cmps: Rc<Cell<usize>>,
// }

// impl<T: PartialEq> PartialEq for SortEvaluator<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.cmps.set(self.cmps.get() + 1);
//         self.t == other.t
//     }
// }
// impl<T: Eq> Eq for SortEvaluator<T> {}

// impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.cmps.set(self.cmps.get() + 1);
//         self.t.partial_cmp(&other.t)
//     }
// }
// impl<T: Ord> Ord for SortEvaluator<T> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cmps.set(self.cmps.get() + 1);
//         self.t.cmp(&other.t)
//     }
// }

// impl<T> Bytify for SortEvaluator<T>
// where
//     T: Bytify,
// {
//     fn bytify(&self, level: usize) -> Option<usize> {
//         return self.t.bytify(level);
//     }
// }

// fn main() {
//     let mut rand = rand::thread_rng();
//     let counter = Rc::new(Cell::new(0));

//     println!("algorithm n comparisons time");
//     for &n in &[0, 1, 10, 100, 1000, 10000, 50000] {
//         let mut values = Vec::with_capacity(n);
//         for _ in 0..n {
//             values.push(SortEvaluator {
//                 t: rand.gen::<usize>(),
//                 cmps: Rc::clone(&counter),
//             });
//         }

//         for _ in 0..10 {
//             values.shuffle(&mut rand);

//             let took = bench(BubbleSort, &values, &counter);
//             println!("{} {} {} {}", "bubble", n, took.0, took.1);
//             let took = bench(InsertionSort { smart: true }, &values, &counter);
//             println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
//             let took = bench(InsertionSort { smart: false }, &values, &counter);
//             println!("{} {} {} {}", "insertion-dumb", n, took.0, took.1);
//             let took = bench(SelectionSort, &values, &counter);
//             println!("{} {} {} {}", "selection", n, took.0, took.1);
//             let took = bench(QuickSort, &values, &counter);
//             println!("{} {} {} {}", "quick", n, took.0, took.1);
//             let took = bench(RadixSort, &values, &counter);
//             println!("{} {} {} {}", "radix", n, took.0, took.1);
//             let took = bench(HeapSort, &values, &counter);
//             println!("{} {} {} {}", "heap", n, took.0, took.1);
//             let took = bench(StdSorter, &values, &counter);
//             println!("{} {} {} {}", "stdstable", n, took.0, took.1);
//             let took = bench(StdUnstableSorter, &values, &counter);
//             println!("{} {} {} {}", "stdunstable", n, took.0, took.1);
//         }
//     }
// }

// fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
//     sorter: S,
//     values: &[SortEvaluator<T>],
//     counter: &Cell<usize>,
// ) -> (usize, f64) {
//     let mut values: Vec<_> = values.to_vec();
//     counter.set(0);
//     let time = std::time::Instant::now();
//     sorter.sort(&mut values);
//     let took = time.elapsed();
//     let count = counter.get();
//     // assert!(values.is_sorted());
//     for i in 1..values.len() {
//         assert!(values[i] >= values[i - 1]);
//     }
//     (count, took.as_secs_f64())
// }
