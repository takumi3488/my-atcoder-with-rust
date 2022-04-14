struct SegmentTree<F, T> {
    size: usize,
    tree: Vec<T>,
    element: T,
    eval: F,
}

impl<F: Fn(T, T) -> T, T: Copy + Eq + std::fmt::Debug> SegmentTree<F, T> {
    fn new(max: usize, element: T, eval: F) -> Self {
        let size = max.next_power_of_two();
        Self {
            size,
            tree: vec![element; size * 2],
            element,
            eval,
        }
    }

    fn get(&self, left: usize, right: usize) -> T {
        return self._get(left, right + 1, 1, 0, self.size);
    }

    fn _get(&self, left: usize, right: usize, now_pos: usize, l: usize, r: usize) -> T {
        if r <= left || right <= l {
            self.element
        } else if left <= l && r <= right {
            self.tree[now_pos]
        } else {
            (self.eval)(
                self._get(left, right, now_pos * 2, l, (l + r) / 2),
                self._get(left, right, now_pos * 2 + 1, (l + r) / 2, r),
            )
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        let mut i = self.size + index;
        while i != 0 {
            let before = self.tree[i];
            let after = (self.eval)(before, value);
            if before == after {
                break;
            }
            self.tree[i] = after;
            i /= 2;
        }
    }
}
