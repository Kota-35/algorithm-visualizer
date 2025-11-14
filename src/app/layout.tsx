import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Algorithm Visualizer - Learn Data Structures & Algorithms",
  description:
    "Interactive visualization platform for algorithms including B-trees, sorting algorithms, graph traversal, and more. Perfect for learning computer science",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={`font-sans antialiased`}>{children}</body>
    </html>
  );
}
