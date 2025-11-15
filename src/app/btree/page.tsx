import clsx from "clsx";
import { BTreeVisualizer } from "./components/BTreeVisualizer";

const BTree = () => {
  return (
    <main
      className={clsx(
        "min-h-screen",
        "bg-background",
        "text-foreground",
        "p-8",
      )}
    >
      <div>
        <BTreeVisualizer />
      </div>
    </main>
  );
};

export default BTree;
