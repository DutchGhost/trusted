use crate::container::container::scope;

pub fn copy<T: Copy>(src: &[T], dst: &mut [T]) {
    scope(src, |src| {
        scope(dst, |mut dst| {
            for (src_idx, dst_idx) in src.zipped(&dst) {
                dst[dst_idx] = src[src_idx];
                // dst[src_idx] = src[dst_idx]; <-- fails to compile, the indices are swapped, and dont belong to the container!!
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_copy() {
        let src = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut dst = [0; 10];

        copy(&src, &mut dst);

        assert_eq!(dst, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
