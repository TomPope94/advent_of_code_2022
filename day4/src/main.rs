use itertools::Itertools;
use std::ops::RangeInclusive;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    let data = include_str!("sectionPairs.txt");

    let assigments = data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse::<u16>().expect("Test"))
                        .collect_tuple::<(u16, u16)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and an end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        })
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();

    println!("{}", assigments);
}
