use std::mem::MaybeUninit;
use std::array;

pub fn split_array<const LEFT: usize, const RIGHT: usize, const LEN: usize, T>(
    arr: [T; LEN],
) -> ([T; LEFT], [T; RIGHT]) {
    assert_eq!(LEFT + RIGHT, LEN);
    let mut left: [MaybeUninit<T>; LEFT] = array::from_fn(|_| MaybeUninit::uninit());
    let mut right: [MaybeUninit<T>; RIGHT] = array::from_fn(|_| MaybeUninit::uninit());

    for (i, val) in arr.into_iter().enumerate() {
        if i < LEFT {
            left[i] = MaybeUninit::new(val);
        } else {
            right[i - LEFT] = MaybeUninit::new(val);
        }
    }

    (
        left.map(|val| unsafe { val.assume_init() }),
        right.map(|val| unsafe { val.assume_init() }),
    )
}