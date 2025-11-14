import clsx from "clsx";
import { HeroSection } from "./components/HeroSection";

export default function Home() {
  return (
    <main className={clsx("min-h-screen", "bg-background")}>
      <HeroSection />
    </main>
  );
}
