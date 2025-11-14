import clsx from "clsx";

export const HeroSection = () => {
  return (
    <section
      className={clsx(
        "relative", // position: relative 配下の要素を相対的な位置に配置
        "overflow-hidden",
        "pt-20",
        "pb-24",
        "sm:pt-32",
        "sm:pb-32",
      )}
    >
      {/* Background Grid */}
      <div className={clsx("absolute", "inset-0", "z-10")}>
        <div
          className={clsx(
            "absolute",
            "inset-0",
            "bg-linear-to-br",
            "from-primary/5",
            "via-transparent",
            "to-accent/5",
          )}
        />
        <svg
          className={clsx(
            "absolute",
            "inset-0",
            "w-full",
            "h-full",
            "opacity-5",
          )}
          preserveAspectRatio="none"
        >
          <title>Background Grid</title>
          <defs>
            <pattern
              id="grid"
              width="40"
              height="40"
              patternUnits="userSpaceOnUse"
            >
              <path
                d="M 40 0 L 0 0 0 40"
                fill="none"
                stroke="currentColor"
                strokeWidth="0.5"
              />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid)" />
        </svg>
      </div>

      <div className="text-center">
        <h1 className="text-5xl  font-bold tracking-tight text-balance mb-6">
          Master Algorithms with
          <span className="block bg-linear-to-r from-primary via-accent to-secondary bg-clip-text text-transparent">
            Visual Learning
          </span>
        </h1>
      </div>
    </section>
  );
};
