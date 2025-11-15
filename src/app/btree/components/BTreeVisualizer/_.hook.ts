import { useCallback, useState } from "react";
import { BTree } from "@/lib/btree";

interface AnimationStep {
  type: "insert" | "delete" | "search" | "split";
  message: string;
  highlightedNode?: number;
  highlightedKey?: number;
}

export const useBTreeVisualizer = () => {
  const [tree] = useState(() => new BTree(3));
  const [steps, setSteps] = useState<AnimationStep[]>([]);
  const [currentStep, setCurrentStep] = useState(0);
  const [isAnimating, setIsAnimating] = useState(false);
  const [inputValue, setInputValue] = useState("");

  const addStep = useCallback((step: AnimationStep) => {
    setSteps((prev) => [...prev, step]);
  }, []);

  const insertKey = () => {
    const key = parseInt(inputValue, 10);
    if (Number.isNaN(key)) return;

    setIsAnimating(true);
    const insertSteps: AnimationStep[] = [];

    insertSteps.push({
      type: "insert",
      message: `キー ${key} を挿入します...`,
    });

    tree.insert(key);

    insertSteps.push({
      type: "insert",
      message: `キー ${key} が正常に挿入されました`,
    });

    setSteps(insertSteps);
    setCurrentStep(0);
    setInputValue("");
    setIsAnimating(false);
  };

  const deleteKey = () => {
    const key = parseInt(inputValue, 10);
    if (Number.isNaN(key)) return;

    setIsAnimating(true);
    const deleteSteps: AnimationStep[] = [];

    deleteSteps.push({
      type: "delete",
      message: `キー ${key} を削除します...`,
    });

    tree.delete(key);

    deleteSteps.push({
      type: "delete",
      message: `キー ${key} が正常に削除されました`,
    });

    setSteps(deleteSteps);
    setCurrentStep(0);
    setInputValue("");
    setIsAnimating(false);
  };

  const searchKey = () => {
    const key = parseInt(inputValue, 10);
    if (Number.isNaN(key)) return;

    setIsAnimating(true);
    const searchSteps: AnimationStep[] = [];

    searchSteps.push({
      type: "search",
      message: `キー ${key} を検索中...`,
    });

    const found = tree.search(key);

    searchSteps.push({
      type: "search",
      message: found
        ? `キー ${key} が見つかりました!`
        : `キー ${key} は見つかりません`,
    });

    setSteps(searchSteps);
    setCurrentStep(0);
    setIsAnimating(false);
  };

  return {
    tree,
    steps,
    currentStep,
    setCurrentStep,
    isAnimating,
    inputValue,
    setInputValue,
    insertKey,
    deleteKey,
    searchKey,
    addStep,
  };
};
