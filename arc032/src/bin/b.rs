use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        uf.unite(a-1, b-1);
    }
    println!("{}", uf.groups_count()-1);
}

struct UnionFind {
    par: Vec<isize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![-1; n], // 親を返す(いなければ-1を返す)
            siz: vec![1; n],  // 根がiのグループのサイズをsiz[i]に格納
        }
    }

    // 根を返す
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == -1 {
            return x;
        }
        self.par[x] = self.root(self.par[x] as usize) as isize;
        self.par[x] as usize
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

        self.par[child] = parent as isize;
        self.siz[parent] += self.siz[child];
        true
    }

    // グループ数をカウント
    fn groups_count(&mut self) -> usize {
        let mut res = 0;
        for p in &self.par {
            if *p == -1 {
                res += 1;
            }
        }
        res
    }
}
