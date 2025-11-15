use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
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

    pub fn keys(&self) -> Vec<i32> {
        self.keys.clone()
    }

    pub fn leaf(&self) -> bool {
        self.leaf
    }

    pub fn children(&self) -> Vec<Box<BTreeNode>> {
        self.children.clone()
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

    /// キーkを削除
    pub fn delete(&mut self, k: i32) -> bool {
        // キーkが存在するか確認
        let idx = self.find_key_index(k);

        if idx < self.keys.len() && self.keys[idx] == k {
            // キーが見つかった
            if self.leaf {
                // 葉ノードの場合、直接削除
                self.keys.remove(idx);
                true
            } else {
                // 内部ノードの場合
                self.delete_from_internal_node(idx)
            }
        } else {
            // キーが見つからない
            if self.leaf {
                false
            } else {
                // 子ノードで削除を試みる
                self.delete_from_subtree(idx, k)
            }
        }
    }

    /// キーのインデックスを見つける
    fn find_key_index(&self, k: i32) -> usize {
        let mut i = 0;
        while i < self.keys.len() && k > self.keys[i] {
            i += 1;
        }
        i
    }

    /// 内部ノードからキーを削除
    fn delete_from_internal_node(&mut self, idx: usize) -> bool {
        // 左の子が十分なキーを持っている場合、前駆で置き換え
        if self.children[idx].keys.len() >= self.t {
            let predecessor = self.get_predecessor(idx);
            self.keys[idx] = predecessor;
            self.children[idx].delete(predecessor)
        }
        // 右の子が十分なキーを持っている場合、後継で置き換え
        else if self.children[idx + 1].keys.len() >= self.t {
            let successor = self.get_successor(idx);
            self.keys[idx] = successor;
            self.children[idx + 1].delete(successor)
        }
        // どちらも十分でない場合、マージしてから削除
        else {
            self.merge_children(idx);
            self.children[idx].delete(self.keys[idx])
        }
    }

    /// 前駆（predecessor）を取得
    fn get_predecessor(&self, idx: usize) -> i32 {
        let mut node = &self.children[idx];
        while !node.leaf {
            node = &node.children[node.children.len() - 1];
        }
        node.keys[node.keys.len() - 1]
    }

    /// 後継（successor）を取得
    fn get_successor(&self, idx: usize) -> i32 {
        let mut node = &self.children[idx + 1];
        while !node.leaf {
            node = &node.children[0];
        }
        node.keys[0]
    }

    /// サブツリーからキーを削除
    fn delete_from_subtree(&mut self, idx: usize, k: i32) -> bool {
        // 子ノードが最小キー数未満の場合、補強する
        let children_len_before = self.children.len();
        if self.children[idx].keys.len() < self.t {
            self.fill_child(idx);
        }

        // マージが発生した場合、インデックスを調整
        // fill_childがマージを実行した場合、children配列のサイズが変わる可能性がある
        let actual_idx = if self.children.len() < children_len_before {
            // マージによりchildren配列のサイズが減った場合
            // idxが範囲外になっている可能性があるので、適切なインデックスを計算
            if idx >= self.children.len() {
                // idxが範囲外の場合、最後の子を使用
                self.children.len() - 1
            } else if idx > 0 && idx < self.keys.len() {
                // マージがidx-1で発生した可能性があるので、idx-1を使用
                idx - 1
            } else {
                idx
            }
        } else if idx > 0
            && idx < self.keys.len()
            && idx < self.children.len()
            && self.children[idx].keys.len() < self.t
        {
            idx - 1
        } else {
            // idxが範囲外の場合は最後の子を使用
            if idx >= self.children.len() {
                self.children.len() - 1
            } else {
                idx
            }
        };

        self.children[actual_idx].delete(k)
    }

    /// 子ノードを補強する（兄弟から借りるかマージする）
    fn fill_child(&mut self, idx: usize) {
        // 前の兄弟から借りる
        if idx != 0 && self.children[idx - 1].keys.len() >= self.t {
            self.borrow_from_prev(idx);
        }
        // 次の兄弟から借りる
        else if idx < self.children.len() - 1
            && self.children[idx + 1].keys.len() >= self.t
        {
            self.borrow_from_next(idx);
        }
        // どちらも借りられない場合、マージ
        else {
            if idx != self.children.len() - 1 {
                self.merge_children(idx);
            } else {
                self.merge_children(idx - 1);
            }
        }
    }

    /// 前の兄弟からキーを借りる
    fn borrow_from_prev(&mut self, idx: usize) {
        let (left, right) = self.children.split_at_mut(idx);
        let sibling = &mut left[idx - 1];
        let child = &mut right[0];

        // 親のキーを子に移動
        child.keys.insert(0, self.keys[idx - 1]);

        // 兄弟の最後の子を子の最初に移動
        if !child.leaf {
            let last_child = sibling.children.pop().unwrap();
            child.children.insert(0, last_child);
        }

        // 兄弟の最後のキーを親に移動
        self.keys[idx - 1] = sibling.keys.pop().unwrap();
    }

    /// 次の兄弟からキーを借りる
    fn borrow_from_next(&mut self, idx: usize) {
        let (left, right) = self.children.split_at_mut(idx + 1);
        let child = &mut left[idx];
        let sibling = &mut right[0];

        // 親のキーを子に移動
        child.keys.push(self.keys[idx]);

        // 兄弟の最初の子を子の最後に移動
        if !child.leaf {
            let first_child = sibling.children.remove(0);
            child.children.push(first_child);
        }

        // 兄弟の最初のキーを親に移動
        self.keys[idx] = sibling.keys.remove(0);
    }

    /// 2つの子ノードをマージ
    fn merge_children(&mut self, idx: usize) {
        let mut child = self.children.remove(idx);
        let sibling = self.children.remove(idx);
        let key = self.keys.remove(idx);

        // 親のキーを子に移動
        child.keys.push(key);

        // 兄弟のキーを子に移動
        child.keys.extend(sibling.keys);

        // 兄弟の子を子に移動
        if !child.leaf {
            child.children.extend(sibling.children);
        }

        // マージした子を配置
        self.children.insert(idx, child);
    }
}
