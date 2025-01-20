import type { Metadata } from "next";
import { Geist } from "next/font/google";
import "./globals.css";
import { StarknetProvider } from "./components/starknet/StarknetProvider";
const geist = Geist({
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Starknet Agent",
  description: "Your AI assistant for Starknet",
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