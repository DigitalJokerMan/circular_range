use std::{iter::Sum, ops::AddAssign};

use num_traits::{PrimInt, Signed};

pub fn circular_range<T: PrimInt + Signed, const N: usize>(radius: [T; N]) -> Vec<[T; N]> {
    // Figure out the capacity by multiplying the squared radii together.
    let mut capacity = 0;
    for i in 0..N {
        capacity += radius[i].pow(2).to_usize().unwrap();
    }

    // Create a vector of the capacity size.
    let mut vec: Vec<[T; N]> = Vec::with_capacity(capacity);

    for i in 0..N {
        for j in -radius[i]..=radius[i] {
            vec.get_mut()
        }
    }

    vec
}

// Generic function to get the distance between two points
pub fn distance<T: PrimInt + Signed + AddAssign, const N: usize>(a: [T; N], b: [T; N]) -> T {
    let mut dist = T::zero();
    for i in 0..N {
        dist += (a[i] - b[i]).abs();
    }
    dist
}
