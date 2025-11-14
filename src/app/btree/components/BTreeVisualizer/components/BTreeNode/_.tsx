import clsx from "clsx";
import type { BTreeNodeType } from "@/lib/btree";

interface BTreeNodeProps {
  node: BTreeNodeType | null;
  depth: number;
}

export const BTreeNode = (props: BTreeNodeProps) => {
  const { node, depth } = props;

  if (!node) return;

  return (
    <div className={clsx("flex", "flex-col", "items-center")}>
      <div className={clsx("mb-8")}>
        <div
          className={clsx(
            "flex",
            "gap-1",
            "bg-accent",
            "rounded-lg",
            "p-3",
            "border-2",
            "shadow-lg",
          )}
        >
          {node.keys.map((key, index) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: とりあえず許可
            <div key={index} className={clsx("flex", "items-center")}>
              <div
                className={clsx(
                  "min-w-8",
                  "h-8",
                  "rounded-md",
                  "bg-accent-foreground",
                  "text-accent",
                  "font-bold",
                  "flex",
                  "items-baseline",
                  "justify-center",
                  "text-sm",
                )}
              >
                {key}
              </div>
              {index < node.keys.length - 1 && (
                <div className={clsx("mx-1", "text-accent-foreground")}>|</div>
              )}
            </div>
          ))}
        </div>
      </div>

      {!node.isLeaf && node.children && node.children.length > 0 && (
        <div className={clsx("flex", "gap-16", "relative")}>
          {node.children.map((child, index) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: とりあえず許可
            <div className="relative" key={index}>
              <div
                className={clsx(
                  "absolute",
                  "bottom-full",
                  "left-1/2",
                  "w-0.5",
                  "bg-border",
                  "h-8",
                  "-translate-x-1/2",
                )}
              />
              <BTreeNode node={child} depth={depth + 1} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
