/// Allows you to iterate from one float value to another.
///
#[derive(Clone)]
pub struct FloatRangeIter {
    start: f64,
    end: f64,
    step: f64,
    current: i64,
    size: i64,
}

/// Converts values to float ranges.
pub trait ToFloatRangeIter {
    fn by(self, step: f64) -> FloatRangeIter;
}

impl Iterator for FloatRangeIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current < self.size {
            let value = self.start + self.step * (self.current as f64);
            assert!(value >= self.start);
            assert!(value < self.end);
            Some(value)
        } else {
            None
        }
    }
}

impl ToFloatRangeIter for std::ops::Range<f64> {
    fn by(self, step: f64) -> FloatRangeIter {
        let std::ops::Range { start, end } = self;
        let size = (end - start) / step;

        FloatRangeIter {
            start,
            end,
            step,
            current: 0,
            size: size as i64,
        }
    }
}
