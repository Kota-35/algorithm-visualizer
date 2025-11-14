pub struct BTreeNode {
    /// キーの配列
    keys: Vec<i32>,

    /// 子ノードへのポインタ配列
    children: Vec<Box<BTreeNode>>,

    /// 最小次数
    t: usize,

    /// 葉ノードかどうか
    leaf: bool,
}

impl BTreeNode {
    pub fn new(t: usize, leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::with_capacity(2 * t - 1),
            children: Vec::with_capacity(2 * t),
            t,
            leaf,
        }
    }

    pub fn traverse(&self) {
        let mut i = 0;
        // n子のキーとn+1個の子ノードを捜査
        while i < self.keys.len() {
            // 葉ノードでない場合、keys[i]を表示する前に
            // 子ノードC[i]を捜査
            if !self.leaf && i < self.children.len() {
                self.children[i].traverse();
            }
            print!(" {}", self.keys[i]);
            i += 1;
        }
    }

    pub fn search(&self, k: i32) -> Option<&BTreeNode> {
        // k以上の最初にキーを探す
        let mut i = 0;
        while i < self.keys.len() && k > self.keys[i] {
            i += 1;
        }

        // 見つかったキーがkと等しい場合、このノードを返す
        if i < self.keys.len() && self.keys[i] == k {
            return Some(self);
        }

        // キーが見つからず、葉ノードの場合
        if self.leaf {
            return None;
        }

        // 適切な子ノードに移動
        if i < self.children.len() {
            self.children[i].search(k)
        } else {
            None
        }
    }

    pub fn insert_not_full(&mut self, k: i32) {
        let mut i = self.keys.len() as i32 - 1;

        if self.leaf {
            // 葉ノードの場合
            // 新しいキーを挿入する位置を見つけ
            // より大きいキーを1つ先に移動
            self.keys.push(0); // スペースを確保 
            while i >= 0 && self.keys[i as usize] > k {
                self.keys[(i + 1) as usize] = self.keys[i as usize];
                i -= 1;
            }

            // 見つかった位置に新しいキーを挿入
            self.keys[(i + 1) as usize] = k;
        } else {
            // 葉ノードではない場合
            // 新しいキーを持つ子ノードを見つける
            while i >= 0 && self.keys[i as usize] > k {
                i -= 1;
            }

            let child_idx = (i + 1) as usize;

            // 見つかった子ノードが満杯かチェック
            if self.children[child_idx].keys.len() == 2 * self.t - 1 {
                // 子ノードが満杯の場合、分割
                self.split_child(child_idx);

                // 分割後、C[i]の中央のキーが上に移動し、
                // C[i]が二つに分割される。どちらが新しいキーを
                // 持つか判断
                if self.keys[child_idx] < k {
                    i += 1;
                }
            }
            self.children[(i + 1) as usize].insert_not_full(k);
        }
    }

    pub fn split_child(&mut self, i: usize) {
        let t = self.t;
        let y = &mut self.children[i];

        // yの(t-1)個のキーを格納する新しいノードを作成
        let mut z = Box::new(BTreeNode::new(t, y.leaf));

        // yの最後の(t-1)個のキーをzにコピー
        z.keys = y.keys.split_off(t);

        // yの最後のt個の子ノードをzにコピー
        if !y.leaf {
            z.children = y.children.split_off(t);
        }

        // yのキーの数を減らす(中央のキーを取り出す)
        let middle_key = y.keys.pop().unwrap();

        // このノードに新しい子ノードを追加するスペースを作成
        self.children.insert(i + 1, z);

        // yの中央のキーをこのノードに移動
        self.keys.insert(i, middle_key);
    }

    /// ノードが満杯かどうか判定
    pub fn is_full(&self) -> bool {
        self.keys.len() == 2 * self.t - 1
    }

    /// キーの数を取得
    pub fn keys_len(&self) -> usize {
        self.keys.len()
    }

    /// 指定されたインデックスのキーを取得
    pub fn get_key(&self, index: usize) -> Option<i32> {
        self.keys.get(index).copied()
    }

    /// 最初のキーを取得(存在する場合)
    pub fn first_key(&self) -> Option<i32> {
        self.keys.first().copied()
    }

    /// 子ノードを追加
    pub fn add_child(&mut self, child: Box<BTreeNode>) {
        self.children.push(child);
    }

    /// 指定されたインデックスの子ノードへの可変参照を取得
    pub fn get_child_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Box<BTreeNode>> {
        self.children.get_mut(index)
    }
}
