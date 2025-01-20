import type { Metadata } from "next";
import { Geist } from "next/font/google";
import "./globals.css";
import { StarknetProvider } from "./components/starknet/StarknetProvider";
const geist = Geist({
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Brother Yields",
  description: "Your AI assistant for customized DeFi strategy Starknet",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <StarknetProvider>
          {children}
        </StarknetProvider>
      </body>
    </html>
  );
}