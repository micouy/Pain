use std::{
    cmp::Ord,
    ops::{Range, RangeInclusive},
};

pub fn range<T>(a: T, b: T) -> Range<T>
where
    T: Ord + Copy,
{
    let min = a.min(b);
    let max = a.max(b);

    min..max
}

pub fn range_inclusive<T>(a: T, b: T) -> RangeInclusive<T>
where
    T: Ord + Copy,
{
    let min = a.min(b);
    let max = a.max(b);

    min..=max
}
