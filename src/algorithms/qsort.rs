use crate::{
    container::container::{scope, Container},
    fundemental::{index::Index, proof::NonEmpty, range::Range},
};

pub fn qsort<T: Ord>(slice: &mut [T]) {
    scope(slice, |mut v| {
        let range = v.range();

        if range.len() > 1 {
            // we already know it is non-empty, so call the faster way.
            let range = unsafe { range.nonempty_unchecked() };
            let p = partition(&mut v, range);

            qsort(&mut v[..p]);
            qsort(&mut v[p..]);
        }
    })
}

fn _quicksort<'id, T: Ord>(v: &mut Container<'id, &mut [T]>, range: Range<'id, NonEmpty>) {
    // There is nothing to sort if the range has a lenght of 1.
    // A range with length of 0 is *impossible* to get here (well, kinda..),
    if range.len() > 1 {
        let p = partition(v, range);
        let (lhs, rhs) = range.split_index(p);
        // We splitted the range at `p`,
        // this means the NonEmpty proof transferred to `rhs`.
        // We must convert `lhs` back into a non-empty range,
        // which should be okey,
        // because if we splitted at idx 0, the recursive call doesn't access the range.
        _quicksort(v, unsafe { lhs.nonempty_unchecked() });
        _quicksort(v, rhs);
    }
}

fn partition<'id, T: Ord>(
    v: &mut Container<'id, &mut [T]>,
    mut range: Range<'id, NonEmpty>,
) -> Index<'id, NonEmpty> {
    let (l, m, r) = (range.first(), range.upper_middle(), range.last());

    let mut pivot = if v[l] <= v[m] && v[m] <= v[r] {
        m
    } else if v[m] <= v[l] && v[l] <= v[r] {
        l
    } else {
        r
    };

    v.swap(range.first(), pivot);
    pivot = range.first();
    'main: loop {
        if v[range.first()] >= v[pivot] {
            loop {
                if v[range.last()] <= v[pivot] {
                    v.swap(range.first(), range.last());
                    break;
                }

                if !range.advance_back() {
                    break 'main;
                }
            }
        }
        if !range.advance() {
            break;
        }
    }

    debug_assert!(range.first() <= range.last());
    range.first()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_reverted_slice() {
        let mut s = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        qsort(&mut s);
        assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
