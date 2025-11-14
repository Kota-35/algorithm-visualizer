import { BTree as WasmBTree } from "@/lib/wasm";

export interface BTreeNodeType {
  keys: number[];
  children: (BTreeNodeType | null)[];
  isLeaf: boolean;
}

export class BTree {
  private wasmBTree: WasmBTree;

  constructor(t: number) {
    this.wasmBTree = new WasmBTree(t);
  }

  insert(key: number): void {
    this.wasmBTree.insert(key);
  }

  search(key: number): boolean {
    return this.wasmBTree.search(key);
  }

  traverse(): void {
    this.wasmBTree.traverse();
  }

  // WASMからツリー構造を取得してTypeScriptの型に変換
  get root(): BTreeNodeType | null {
    const structure = this.wasmBTree.get_structure();
    if (!structure) return null;
    return this.jsValueToNode(structure);
  }

  // biome-ignore lint/suspicious/noExplicitAny: WASMの使用で戻り値はAnyになるため
  private jsValueToNode(jsValue: any): BTreeNodeType | null {
    if (!jsValue) return null;

    const keys: number[] = Array.from(jsValue.keys || []);
    const children: (BTreeNodeType | null)[] = [];

    if (jsValue.children) {
      for (let i = 0; i < jsValue.children.length; i++) {
        children.push(this.jsValueToNode(jsValue.children[i]));
      }
    }

    return {
      keys,
      children,
      isLeaf: jsValue.isLeaf || false,
    };
  }

  getTotalKeys(): number {
    return this.wasmBTree.get_total_keys();
  }

  getHeight(): number {
    return this.wasmBTree.get_height();
  }
}
