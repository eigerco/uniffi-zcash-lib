pub(crate) fn cast_slice<T, const N: usize>(slice: &[T]) -> crate::ZcashResult<[T; N]>
where
    T: Copy,
{
    slice
        .try_into()
        .map_err(|_| crate::ZcashError::ArrayLengthMismatch {
            expected: N as u64,
            got: slice.len() as u64,
        })
}
