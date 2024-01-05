trait DSU<T> {
    fn find(&mut self, node: T) -> T;
    fn union(&mut self, x: T, y: T);
}

macro_rules! impl_dsu {
    ($t:ty) => {
        impl DSU<$t> for Vec<$t> {
            fn find(&mut self, node: $t) -> $t {
                if node == self[node as usize] {
                    return node;
                }
                self[node as usize] = self.find(self[node as usize]);
                return self[node as usize];
            }
            fn union(&mut self, mut x: $t, mut y: $t) {
                x = self.find(x);
                y = self.find(y);
                if x != y {
                    self[y as usize] = x;
                }
            }
        }
    };
}

// impl_dsu!(u8);
// impl_dsu!(u16);
impl_dsu!(u32);
// impl_dsu!(u64);
// impl_dsu!(u128);
