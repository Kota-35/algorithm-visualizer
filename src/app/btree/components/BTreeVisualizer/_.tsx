"use client";

import clsx from "clsx";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/ui/Input";
import { useBTreeVisualizer } from "./_.hook.ts";
import { BTreeNode } from "./components/BTreeNode/_.tsx";

export const BTreeVisualizer = () => {
  const {
    tree,
    steps,
    isAnimating,
    inputValue,
    currentStep,
    insertKey,
    deleteKey,
    searchKey,
    setInputValue,
    setCurrentStep,
  } = useBTreeVisualizer();

  return (
    <div className={clsx("space-y-6")}>
      <div className="space-y-4">
        <div className={clsx("flex", "gap-2")}>
          <Input
            type="number"
            disabled={isAnimating}
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyPress={(e) => e.key === "Enter" && insertKey()}
          />
          <Button onClick={insertKey} disabled={isAnimating || !inputValue}>
            挿入
          </Button>
          <Button onClick={deleteKey} disabled={isAnimating || !inputValue}>
            削除
          </Button>
          <Button onClick={searchKey} disabled={isAnimating || !inputValue}>
            検索
          </Button>
        </div>

        {steps.length > 0 && (
          <div className={clsx("bg-muted", "p-4", "rounded-md")}>
            <p className={clsx("text-sm", "text-muted-foreground", "mb-2")}>
              ステップ {currentStep + 1} / {steps.length}
            </p>
            <p className={clsx("text-foreground", "font-medium", "mb-3")}>
              {steps[currentStep]?.message}
            </p>
            <div className="flex gap-2">
              <Button
                onClick={() => setCurrentStep(Math.max(0, currentStep - 1))}
                disabled={currentStep === 1}
              >
                <ArrowLeft className="w-2 h-2" /> 前へ
              </Button>
              <Button
                onClick={() =>
                  setCurrentStep(Math.min(steps.length - 1, currentStep + 1))
                }
                disabled={currentStep === steps.length - 1}
              >
                次へ <ArrowRight className="w-2 h-2" />
              </Button>
            </div>
          </div>
        )}
      </div>

      <div
        className={clsx(
          "bg-muted/50",
          "p-6",
          "rounded-md",
          "border",
          "border-border",
          "overflow-x-auto",
          "min-h-96",
        )}
      >
        <div className={clsx("flex", "justify-center", "items-start")}>
          {tree.root ? (
            <BTreeNode node={tree.root} depth={0} />
          ) : (
            <div
              className={clsx("text-center", "text-muted-foreground", "py-20")}
            >
              <p className={clsx("text-lg")}>木が空です</p>
              <p className={clsx("text-sm")}>キーを挿入して始めましょう</p>
            </div>
          )}
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4 text-sm">
        <div className="bg-card border border-border rounded-md p-3">
          <p className="text-muted-foreground text-xs mb-1">次数 (t)</p>
          <p className="text-lg font-bold text-accent">3</p>
        </div>
        <div className="bg-card border border-border rounded-md p-3">
          <p className="text-muted-foreground text-xs mb-1">キー数</p>
          <p className="text-lg font-bold text-accent">{tree.getTotalKeys()}</p>
        </div>
        <div className="bg-card border border-border rounded-md p-3">
          <p className="text-muted-foreground text-xs mb-1">木の高さ</p>
          <p className="text-lg font-bold text-accent">{tree.getHeight()}</p>
        </div>
      </div>
    </div>
  );
};
