import type { Metadata } from "next";
import { Geist } from "next/font/google";
import "./globals.css";

const geist = Geist({
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Starknet Agent",
  description: "Your AI assistant for Starknet",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className={`${geist.className} bg-black text-white antialiased`}>
        <div className="fixed inset-0 bg-gradient-to-b from-black to-zinc-900">
        <div id="particles-js">
        </div>          <div className="absolute inset-0 animate-wave" />
        </div>
        {children}
      </body>
    </html>
  );
}
