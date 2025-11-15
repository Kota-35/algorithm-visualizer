use std::fmt;

use crate::btree::node::BTreeNode;
use js_sys::Array;
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

    fn node_to_js_value(&self, node: &BTreeNode) -> JsValue {
        let obj = js_sys::Object::new();

        // keys配列を作成
        let keys = Array::from_iter(
            node.keys().iter().map(|key| JsValue::from(*key)),
        );
        let _ = js_sys::Reflect::set(&obj, &"keys".into(), &keys.into());

        // children配列の作成
        let children = if !node.leaf() {
            Array::from_iter(
                node.children()
                    .iter()
                    .map(|child| self.node_to_js_value(&child)),
            )
        } else {
            Array::new()
        };

        let _ = js_sys::Reflect::set(
            &obj,
            &"children".into(),
            &children.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"isLeaf".into(),
            &JsValue::from(node.leaf()),
        );

        obj.into()
    }

    #[wasm_bindgen]
    pub fn get_structure(&self) -> JsValue {
        match &self.root {
            Some(root) => self.node_to_js_value(&root),
            None => JsValue::NULL,
        }
    }

    /// キーの総数を取得
    #[wasm_bindgen]
    pub fn get_total_keys(&self) -> usize {
        self.count_keys(&self.root)
    }

    fn count_keys(&self, node: &Option<Box<BTreeNode>>) -> usize {
        match node {
            Some(n) => {
                let keys_count = n.keys_len();
                let children_keys_count = if !n.leaf() {
                    n.children()
                        .iter()
                        .map(|child| self.count_keys(&Some(child.clone())))
                        .sum()
                } else {
                    0
                };

                keys_count + children_keys_count
            }
            None => 0,
        }
    }

    fn get_node_height(&self, node: &Option<Box<BTreeNode>>) -> usize {
        match node {
            Some(n) => {
                if n.leaf() {
                    1
                } else if n.children().is_empty() {
                    1
                } else {
                    1 + self
                        .get_node_height(&Some(n.children()[0].clone()))
                }
            }
            None => 0,
        }
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> usize {
        self.get_node_height(&self.root)
    }

    /// キーkを削除
    #[wasm_bindgen]
    pub fn delete(&mut self, k: i32) -> bool {
        match self.root.take() {
            None => false,
            Some(mut root) => {
                let result = root.delete(k);

                // ルートが空になった場合、最初の子を新しいルートにする
                if root.keys().is_empty() && !root.leaf() {
                    self.root = root.children().pop();
                } else {
                    self.root = Some(root);
                }

                result
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

    #[test]
    fn test_btree_delete_leaf_node() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);
        t.insert(6);
        t.insert(12);
        t.insert(30);
        t.insert(7);
        t.insert(17);

        let initial_keys = t.get_total_keys();
        assert!(t.delete(7), "7 should be deleted successfully");
        assert!(!t.search(7), "7 should not be present after deletion");
        assert_eq!(
            t.get_total_keys(),
            initial_keys - 1,
            "Total keys should decrease by 1"
        );
    }

    #[test]
    fn test_btree_delete_internal_node() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);
        t.insert(6);
        t.insert(12);
        t.insert(30);
        t.insert(7);
        t.insert(17);

        let initial_keys = t.get_total_keys();
        assert!(t.delete(10), "10 should be deleted successfully");
        assert!(!t.search(10), "10 should not be present after deletion");
        assert_eq!(
            t.get_total_keys(),
            initial_keys - 1,
            "Total keys should decrease by 1"
        );

        // 他のキーがまだ存在することを確認
        assert!(t.search(5), "5 should still be present");
        assert!(t.search(20), "20 should still be present");
    }

    #[test]
    fn test_btree_delete_nonexistent_key() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);

        let initial_keys = t.get_total_keys();
        assert!(
            !t.delete(99),
            "99 should not be deleted (does not exist)"
        );
        assert_eq!(
            t.get_total_keys(),
            initial_keys,
            "Total keys should remain the same"
        );
    }

    #[test]
    fn test_btree_delete_multiple_keys() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);
        t.insert(6);
        t.insert(12);
        t.insert(30);
        t.insert(7);
        t.insert(17);

        let initial_keys = t.get_total_keys();

        // 複数のキーを削除
        assert!(t.delete(6), "6 should be deleted");
        assert!(t.delete(12), "12 should be deleted");
        assert!(t.delete(30), "30 should be deleted");

        assert!(!t.search(6), "6 should not be present");
        assert!(!t.search(12), "12 should not be present");
        assert!(!t.search(30), "30 should not be present");
        assert_eq!(
            t.get_total_keys(),
            initial_keys - 3,
            "Total keys should decrease by 3"
        );

        // 残りのキーが存在することを確認
        assert!(t.search(10), "10 should still be present");
        assert!(t.search(20), "20 should still be present");
        assert!(t.search(5), "5 should still be present");
    }

    #[test]
    fn test_btree_delete_all_keys() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);

        assert!(t.delete(10), "10 should be deleted");
        assert!(t.delete(20), "20 should be deleted");
        assert!(t.delete(5), "5 should be deleted");

        assert_eq!(t.get_total_keys(), 0, "Total keys should be 0");
        assert!(!t.search(10), "10 should not be present");
        assert!(!t.search(20), "20 should not be present");
        assert!(!t.search(5), "5 should not be present");
    }

    #[test]
    fn test_btree_delete_with_merge() {
        let mut t = BTree::new(3);

        // マージが発生するような構造を作成
        t.insert(1);
        t.insert(2);
        t.insert(3);
        t.insert(4);
        t.insert(5);
        t.insert(6);
        t.insert(7);
        t.insert(8);
        t.insert(9);
        t.insert(10);

        let initial_keys = t.get_total_keys();

        // 内部ノードから削除してマージを発生させる
        assert!(t.delete(5), "5 should be deleted");
        assert!(!t.search(5), "5 should not be present after deletion");
        assert_eq!(
            t.get_total_keys(),
            initial_keys - 1,
            "Total keys should decrease by 1"
        );

        // ツリーの構造が正しく保たれていることを確認
        assert!(t.search(1), "1 should still be present");
        assert!(t.search(10), "10 should still be present");
    }

    #[test]
    fn test_btree_delete_root_becomes_empty() {
        let mut t = BTree::new(3);

        t.insert(10);
        t.insert(20);
        t.insert(5);
        t.insert(6);
        t.insert(12);
        t.insert(30);
        t.insert(7);
        t.insert(17);

        // ルートが空になるような削除を実行
        // まず、ルート以外のキーを削除してから、ルートのキーを削除
        let root_key = if t.search(10) { 10 } else { 5 };

        // ルートのキーを削除
        assert!(t.delete(root_key), "Root key should be deleted");
        assert!(
            !t.search(root_key),
            "Root key should not be present after deletion"
        );

        // ツリーがまだ有効であることを確認
        let remaining_keys = t.get_total_keys();
        assert!(remaining_keys > 0, "Tree should still have keys");
    }

    #[test]
    fn test_btree_delete_sequential() {
        let mut t = BTree::new(3);

        // 順序よく挿入
        for i in 1..=20 {
            t.insert(i);
        }

        let initial_keys = t.get_total_keys();
        assert_eq!(initial_keys, 20, "Should have 20 keys initially");

        // 順序よく削除
        for i in 1..=10 {
            assert!(t.delete(i), "Key {} should be deleted", i);
            assert!(!t.search(i), "Key {} should not be present", i);
        }

        assert_eq!(
            t.get_total_keys(),
            10,
            "Should have 10 keys remaining"
        );

        // 残りのキーが存在することを確認
        for i in 11..=20 {
            assert!(t.search(i), "Key {} should still be present", i);
        }
    }

    #[test]
    fn test_btree_delete_reverse_sequential() {
        let mut t = BTree::new(3);

        // 順序よく挿入（より小さな範囲でテスト）
        for i in 1..=10 {
            t.insert(i);
        }

        // 逆順で削除
        for i in (1..=10).rev() {
            assert!(t.delete(i), "Key {} should be deleted", i);
            assert!(!t.search(i), "Key {} should not be present", i);
        }

        assert_eq!(t.get_total_keys(), 0, "Should have 0 keys remaining");
    }
}
