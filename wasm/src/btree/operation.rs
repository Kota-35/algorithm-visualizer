use std::fmt;

use crate::btree::node::BTreeNode;
use wasm_bindgen::prelude::*;

// B-Tree
#[wasm_bindgen]
pub struct BTree {
    // 根
    root: Option<Box<BTreeNode>>,

    // 次数
    t: usize,
}

#[wasm_bindgen]
impl BTree {
    #[wasm_bindgen(constructor)]
    pub fn new(t: usize) -> Self {
        BTree { root: None, t }
    }

    // ツリー全体を走査
    pub fn traverse(&self) {
        if let Some(ref root) = self.root {
            root.traverse();
        }
    }

    /// キーkを探索
    pub fn search(&self, k: i32) -> bool {
        match &self.root {
            Some(root) => root.search(k).is_some(),
            None => false,
        }
    }

    pub fn insert(&mut self, k: i32) {
        match self.root.take() {
            None => {
                // ツリーが空の場合
                let mut new_root = Box::new(BTreeNode::new(self.t, true));
                new_root.insert_not_full(k);
                self.root = Some(new_root)
            }
            Some(mut root) => {
                // ルートが満杯の場合、ツリーの高さが増える
                if root.is_full() {
                    // 新しいルートを作成
                    let mut s = Box::new(BTreeNode::new(self.t, false));

                    // 古いルートを新しいルートの子にする
                    s.add_child(root);
                    // 古いルートを分割して、1つのキーを新しいルートに移動
                    s.split_child(0);

                    // 新しいルートには2つの子がある
                    // どちらの子が新しいキーを持つか判断
                    let i = if let Some(first_key) = s.get_key(0) {
                        if first_key < k { 1 } else { 0 }
                    } else {
                        0
                    };
                    if let Some(child) = s.get_child_mut(i) {
                        child.insert_not_full(k);
                    }

                    // ルートを変更
                    self.root = Some(s);
                } else {
                    // ルートが満杯でない場合
                    root.insert_not_full(k);
                    self.root = Some(root)
                }
            }
        }
    }
}

impl fmt::Display for BTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BTree traversal")?;
        if let Some(ref root) = self.root {
            root.traverse();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_insertion_and_search() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);
        t.insert(6);
        t.insert(12);
        t.insert(30);
        t.insert(7);
        t.insert(17);

        print!("Traversal of the constructed tree is");
        t.traverse();
        println!();

        assert!(t.search(6), "6 should be present");
        assert!(!t.search(15), "15 should not be present");
    }
}
