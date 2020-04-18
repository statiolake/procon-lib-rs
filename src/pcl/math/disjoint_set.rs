use std::mem::swap;

pub struct DisjointSet {
    par: Vec<i64>,
    size: usize,
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        DisjointSet {
            par: vec![-1; n],
            size: n,
        }
    }

    /// 二つのグループをマージする。元々同じグループに属していたなら false を返す。
    pub fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        let len = self.par.len();
        assert!(x < len, "index out of range: x is {} but len is {}", x, len);
        assert!(y < len, "index out of range: y is {} but len is {}", y, len);

        x = self.root(x);
        y = self.root(y);
        if x == y {
            return false;
        }

        if self.par[x] < self.par[y] {
            swap(&mut x, &mut y);
        }

        debug_assert!(
            self.par[x] < 0 && self.par[y] < 0,
            "critical error: parent has invalid value for rank"
        );

        self.par[x] += self.par[y];
        self.par[y] = x as i64;
        self.size -= 1;

        true
    }

    pub fn in_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn root(&mut self, x: usize) -> usize {
        let parx = self.par[x];
        if parx < 0 {
            x
        } else {
            let root = self.root(parx as usize);
            self.par[x] = root as i64;
            root
        }
    }

    pub fn size_of(&mut self, mut x: usize) -> usize {
        x = self.root(x);

        debug_assert!(
            self.par[x] < 0,
            "critical error: parent has invalid value for rank"
        );
        -self.par[x] as usize
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disjoint_set() {
        let mut uf = DisjointSet::new(5);

        assert_eq!(uf.size(), 5);
        assert!(!uf.in_same(0, 1));
        assert!(uf.merge(0, 1));
        assert!(uf.in_same(0, 1));
        assert!(!uf.in_same(1, 2));
        assert!(uf.size_of(0) == 2);

        assert_eq!(uf.size(), 4);
        assert!(uf.merge(2, 3));
        assert!(!uf.in_same(1, 2));
        assert!(uf.size_of(2) == 2);

        assert_eq!(uf.size(), 3);
        assert!(uf.merge(1, 3));
        assert!(uf.in_same(1, 2));
        assert!(uf.size_of(2) == 4);

        assert!(!uf.merge(1, 3));
    }
}
