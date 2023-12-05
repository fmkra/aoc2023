use std::cmp::{max,min};

pub struct Range {
    pub intervals: Vec<(i64,i64)>
}

impl Range {
    pub fn new() -> Self {
        Range {
            intervals: Vec::new()
        }
    }
    pub fn add(&mut self, range: (i64,i64)) {
        let mut events: Vec<(i64, bool)> = Vec::new();
        for (p,q) in self.intervals.iter() {
            events.push((*p, false));
            events.push((*q, true));
        }
        events.push((range.0, false));
        events.push((range.1, true));
        events.sort();
        let mut range_start = events[0].0;
        let mut opened_ranges = 1;
        self.intervals = Vec::new();
        for event in events.iter().skip(1) {
            if event.1 == false { // interval begin
                opened_ranges += 1;
                if opened_ranges == 1 {
                    range_start = event.0;
                }
            } else { // interval end
                opened_ranges -= 1;
                if opened_ranges == 0 {
                    self.intervals.push((range_start, event.0));
                }
            }
        }
    }

    pub fn remove(&mut self, range: (i64, i64)) {
        let mut vec: Vec<(i64, i64)> = Vec::new();
        for (p,q) in self.intervals.iter() {
            if range.0 <= *p && *q <= range.1 {
                continue;
            } else if *q <= range.0 || range.1 <= *p {
                vec.push((*p, *q));
            } else if *p < range.0 && range.1 < *q {
                vec.push((*p, range.0));
                vec.push((range.1, *q));
            } else if *p < range.0 && range.0 < *q {
                vec.push((*p, range.0));
            } else if *p < range.1 && range.1 < *q {
                vec.push((range.1, *q));
            } else {
                panic!("unreachable");
            }
        }
        self.intervals = vec;
    }

    pub fn get_intersection(&self, range: (i64, i64)) -> Range {
        let mut vec: Vec<(i64, i64)> = Vec::new();
        for (p,q) in self.intervals.iter() {
            let interval = (max(*p, range.0), min(*q, range.1));
            if interval.0 < interval.1 {
                vec.push(interval);
            }
        }
        Range {
            intervals: vec
        }
    }

    pub fn shift(&mut self, range: (i64, i64), shift: i64) {
        let intersection = self.get_intersection(range);
        self.remove(range);
        for (p,q) in intersection.intervals.iter() {
            self.add((p + shift, q + shift));
        }
    }

    pub fn shift_many(&mut self, ranges: Vec<(i64, i64, i64)>) {
        let mut intersections = Vec::new();
        for (_, from, len) in ranges.iter() {
            intersections.push(self.get_intersection((*from, *from + *len)));
            self.remove((*from, *from + *len));
        }
        for (intersection, (to, from, _)) in intersections.iter().zip(ranges.iter()) {
            let shift = to - from;
            for irange in intersection.intervals.iter() {
                self.add((irange.0 + shift, irange.1 + shift));
            }
        }
    }
}
