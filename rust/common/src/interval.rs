use std::{
    cmp::{max, min},
    fmt::Display,
};

/// Represents the interval `[low, high]`, where each element in the
/// interval is an integer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntInterval {
    low: isize,
    high: isize,
}

impl IntInterval {
    /// Creates a new interval interval representing `[low, high]`.
    ///
    /// # Parameters
    /// - `low`: The lower-bound of the interval.
    /// - `high`: The upper-bound of the interval.
    ///
    /// # Returns
    /// The interval.
    pub fn new(low: isize, high: isize) -> IntInterval {
        IntInterval { low, high }
    }

    /// Gets the length of this interval.
    ///
    /// # Returns
    /// The length of this interval.
    pub fn len(&self) -> usize {
        if self.low > self.high {
            0
        } else {
            (self.high - self.low + 1) as usize
        }
    }

    /// Checks if this interval represents an empty interval.
    ///
    /// # Returns
    /// `true` if no integers are in this interval. This only happens if `low > high`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the value is in the interval.
    ///
    /// # Parameters
    /// - `val`: The value to check.
    ///
    /// # Returns
    /// `true` if the value is in the interval, `false` otherwise.
    pub fn contains(&self, val: isize) -> bool {
        (self.low..=self.high).contains(&val)
    }

    /// Merges this interval with another interval. This is also known as the union of two intervals.
    ///
    /// # Parameters
    /// - `other`: The other interval to merge.
    ///
    /// # Returns
    /// The interval, if merging is possible. Otherwise, `None` is returned.
    pub fn merge(&self, other: &IntInterval) -> Option<IntInterval> {
        let ((left_min, left_max), (right_min, right_max)) = if self.low <= other.low {
            ((self.low, self.high), (other.low, other.high))
        } else {
            ((other.low, other.high), (self.low, self.high))
        };

        if left_max + 1 == right_min || left_max >= right_min {
            Some(IntInterval::new(
                min(left_min, right_min),
                max(left_max, right_max),
            ))
        } else {
            // No overlap
            None
        }
    }

    /// Finds the commonly shared region between two intervals. This is also known as the intersection of two intervals.
    ///
    /// # Parameters
    /// - `other`: The other interval to intersect with.
    ///
    /// # Returns
    /// The interval, if intersection is possible. Otherwise, `None` is returned.
    pub fn intersect(&self, other: &IntInterval) -> Option<IntInterval> {
        let start_pt = max(self.low, other.low);
        let end_pt = min(self.high, other.high);
        if start_pt <= end_pt {
            Some(IntInterval::new(start_pt, end_pt))
        } else {
            None
        }
    }
}

impl Display for IntInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.low, self.high))
    }
}

#[cfg(test)]
mod interval_tests {
    use super::IntInterval;

    #[test]
    fn test_len() {
        let int1 = IntInterval::new(1, 5);
        let int2 = IntInterval::new(250, 50);
        let int3 = IntInterval::new(1, 1);

        assert_eq!(5, int1.len());
        assert_eq!(0, int2.len());
        assert_eq!(1, int3.len());
    }

    #[test]
    fn test_contains() {
        let int1 = IntInterval::new(50, 250);
        let int2 = IntInterval::new(250, 50);

        assert!(int1.contains(50));
        assert!(int1.contains(250));
        assert!(int1.contains(66));
        assert!(!int1.contains(49));
        assert!(!int1.contains(251));

        assert!(!int2.contains(50));
        assert!(!int2.contains(250));
        assert!(!int2.contains(66));
        assert!(!int2.contains(49));
        assert!(!int2.contains(251));
    }

    #[test]
    fn test_merge_sanity() {
        let int1 = IntInterval::new(10, 5);
        let int2 = IntInterval::new(50, 15);
        assert_eq!(None, int1.merge(&int2));
        assert_eq!(None, int2.merge(&int1));
    }

    #[test]
    fn test_merge_no_overlap() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(102, 150);

        assert_eq!(None, int1.merge(&int2));
        assert_eq!(None, int2.merge(&int1));
    }

    #[test]
    fn test_merge_partial_enclosure() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(50, 150);

        assert_eq!(Some(IntInterval::new(0, 150)), int1.merge(&int2));
        assert_eq!(Some(IntInterval::new(0, 150)), int2.merge(&int1));
    }

    #[test]
    fn test_merge_touching_interval() {
        let int1 = IntInterval::new(0, 50);
        let int2 = IntInterval::new(51, 100);
        let int3 = IntInterval::new(50, 100);

        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int2));
        assert_eq!(Some(IntInterval::new(0, 100)), int2.merge(&int1));
        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int3));
        assert_eq!(Some(IntInterval::new(0, 100)), int3.merge(&int1));
    }

    #[test]
    fn test_merge_enclose_general() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(10, 30);

        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int2));
        assert_eq!(Some(IntInterval::new(0, 100)), int2.merge(&int1));
    }

    #[test]
    fn test_merge_enclose_endpoint() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(0, 30);
        let int4 = IntInterval::new(55, 100);

        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int2));
        assert_eq!(Some(IntInterval::new(0, 100)), int2.merge(&int1));
        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int4));
        assert_eq!(Some(IntInterval::new(0, 100)), int4.merge(&int1));
    }

    #[test]
    fn test_merge_identity() {
        let int1 = IntInterval::new(0, 100);
        assert_eq!(Some(IntInterval::new(0, 100)), int1.merge(&int1));
    }

    #[test]
    fn test_intersect_identity() {
        let int1 = IntInterval::new(0, 100);
        assert_eq!(Some(IntInterval::new(0, 100)), int1.intersect(&int1));
    }

    #[test]
    fn test_intersect_enclose() {
        let int1 = IntInterval::new(10, 50);
        let int2 = IntInterval::new(20, 40);

        assert_eq!(Some(IntInterval::new(20, 40)), int1.intersect(&int2));
        assert_eq!(Some(IntInterval::new(20, 40)), int2.intersect(&int1));
    }

    #[test]
    fn test_intersect_enclose_endpoint() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(0, 30);
        let int3 = IntInterval::new(70, 100);

        assert_eq!(Some(IntInterval::new(0, 30)), int1.intersect(&int2));
        assert_eq!(Some(IntInterval::new(0, 30)), int2.intersect(&int1));

        assert_eq!(Some(IntInterval::new(70, 100)), int1.intersect(&int3));
        assert_eq!(Some(IntInterval::new(70, 100)), int3.intersect(&int1));
    }

    #[test]
    fn test_intersect_no_overlap() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(102, 150);

        assert_eq!(None, int1.intersect(&int2));
        assert_eq!(None, int2.intersect(&int1));
    }

    #[test]
    fn test_intersect_partial_enclosure() {
        let int1 = IntInterval::new(0, 100);
        let int2 = IntInterval::new(50, 150);

        assert_eq!(Some(IntInterval::new(50, 100)), int1.intersect(&int2));
        assert_eq!(Some(IntInterval::new(50, 100)), int2.intersect(&int1));
    }
}
