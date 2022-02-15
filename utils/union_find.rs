struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![-1; n], // 親を返す(いなければ-1を返す)
            siz: vec![1; n], // 根がiのグループのサイズをsiz[i]に格納
        }
    }

    // 根を返す
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] ==-1 {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    // 同じ根を持つかを判別
    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // 指定した2要素を含むグループを結合
    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    // その要素を含むグループの要素数をカウント
    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

    // グループ数をカウント
    fn groups_count(&mut self) -> usize {
        let mut res = 0;
        for p in self.par {
            res += p==-1;
        }
        res
    }
}
